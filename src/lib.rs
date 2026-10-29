//! StakeStreak on NEAR.
//!
//! One pot = one Sputnik DAO created through the Sputnik factory. This contract builds the DAO's
//! policy itself, so the policy is the same for every pot and a judge can read it on chain:
//! a single role whose only member is this contract, allowed to file and approve `Transfer`
//! proposals and nothing else. Stakes go into the DAO treasury. Money leaves the DAO only as
//! Transfer proposals that this contract files from its own books (survivors, fee, refunds).
//!
//! Check-ins and the "missed a day = out" rule live here, on the block clock.
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde_json::{self, json, Value};
use near_sdk::store::IterableMap;
use near_sdk::{
    env, near, require, AccountId, Gas, NearToken, PanicOnDefault, Promise, PromiseOrValue,
};

const MAX_MEMBERS: usize = 20;
const MAX_DAYS: u32 = 90;
const MAX_FEE_BPS: u16 = 300;
const MAX_FREEZES: u8 = 2;
/// Kept by this contract out of `create_pot`'s deposit, to pay the storage of the pot record.
const STORAGE_RESERVE: NearToken = NearToken::from_millinear(20);
/// The deployed Sputnik factory refuses less than code_len + 32 bytes of storage (5.3968 NEAR on 29 Oct 2026).
const MIN_DAO_DEPOSIT: NearToken = NearToken::from_millinear(5_400);
/// A Sputnik proposal that nobody approves for a year expires; ours are approved in the same chain.
const PROPOSAL_PERIOD_NS: &str = "31536000000000000";

const GAS_FACTORY_CREATE: Gas = Gas::from_tgas(240);
const GAS_ON_CREATED: Gas = Gas::from_tgas(15);
const GAS_ON_STAKE: Gas = Gas::from_tgas(10);
const GAS_ADD_PROPOSAL: Gas = Gas::from_tgas(15);
const GAS_ACT_PROPOSAL: Gas = Gas::from_tgas(50);
const GAS_ON_APPROVED: Gas = Gas::from_tgas(10);
/// Static gas one payout step reserves in the worst case (file + callback + approve + callback).
const GAS_PER_PAYOUT: Gas = Gas::from_tgas(85);

#[near(serializers = [borsh, json])]
#[derive(Clone, PartialEq, Debug)]
pub enum PayKind {
    Win,
    Fee,
    Refund,
}

#[near(serializers = [borsh, json])]
#[derive(Clone, PartialEq, Debug)]
pub enum PayState {
    /// Nothing filed in the DAO yet.
    Unpaid,
    /// Transfer proposal filed in the DAO under this id, not yet approved by the policy role.
    Filed(u64),
    /// The policy role approved this proposal id; the DAO executes the transfer.
    Approved(u64),
}

#[near(serializers = [borsh, json])]
#[derive(Clone)]
pub struct Payout {
    pub to: AccountId,
    pub amount: U128,
    pub kind: PayKind,
    pub state: PayState,
}

#[near(serializers = [borsh])]
#[derive(Clone)]
pub struct Member {
    pub account: AccountId,
    /// Bit d set = checked in (or froze) on day d.
    pub days: u128,
    pub freezes: u8,
    /// The stake reached the DAO treasury.
    pub funded: bool,
}

#[near(serializers = [borsh])]
pub struct Pot {
    pub dao: AccountId,
    pub creator: AccountId,
    pub fee_account: AccountId,
    pub rule: String,
    pub fee_bps: u16,
    pub stake: u128,
    pub freeze_fee: u128,
    pub duration_days: u32,
    pub day_ns: u64,
    /// Joining closes and day 0 begins here.
    pub start_ns: u64,
    pub min_members: u32,
    /// The DAO exists.
    pub created: bool,
    pub members: Vec<Member>,
    pub payouts: Vec<Payout>,
    pub settled: bool,
}

#[near(serializers = [json])]
#[derive(Clone)]
pub struct CreatePotArgs {
    /// Pot id and DAO sub-account label: `<name>.<factory>`.
    pub name: String,
    pub rule: String,
    pub stake: U128,
    pub duration_days: u32,
    pub day_secs: u32,
    /// Seconds from creation until joining closes and day 0 begins.
    pub join_secs: u32,
    pub min_members: u32,
    pub fee_account: AccountId,
    pub fee_bps: u16,
    pub freeze_fee: U128,
}

#[near(serializers = [json])]
pub struct MemberView {
    pub account: AccountId,
    pub funded: bool,
    pub alive: bool,
    pub checkins: u32,
    pub freezes: u8,
    pub days: U128,
}

#[near(serializers = [json])]
pub struct PotView {
    pub name: String,
    pub dao: AccountId,
    pub creator: AccountId,
    pub fee_account: AccountId,
    pub rule: String,
    pub fee_bps: u16,
    pub stake: U128,
    pub freeze_fee: U128,
    pub duration_days: u32,
    pub day_ns: u64,
    pub start_ns: u64,
    pub min_members: u32,
    pub created: bool,
    /// creating | open | running | ended | settled
    pub phase: String,
    /// Current day index while running, else null.
    pub day: Option<u32>,
    pub members: Vec<MemberView>,
    pub payouts: Vec<Payout>,
    pub payouts_pending: u32,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct StakeStreak {
    factory: AccountId,
    pots: IterableMap<String, Pot>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Phase {
    Open,
    Running(u32),
    Ended,
}

fn phase_of(pot: &Pot, now: u64) -> Phase {
    if now < pot.start_ns {
        return Phase::Open;
    }
    let day = (now - pot.start_ns) / pot.day_ns;
    if day < pot.duration_days as u64 {
        Phase::Running(day as u32)
    } else {
        Phase::Ended
    }
}

/// Days that are over at `now`, capped at the pot length.
fn completed_days(pot: &Pot, now: u64) -> u32 {
    match phase_of(pot, now) {
        Phase::Open => 0,
        Phase::Running(d) => d,
        Phase::Ended => pot.duration_days,
    }
}

/// A member is in while every completed day has its bit set.
pub fn alive(days: u128, completed: u32) -> bool {
    let required = if completed >= 128 {
        u128::MAX
    } else {
        (1u128 << completed) - 1
    };
    days & required == required
}

/// Equal split of the pool after the fee. Returns (fee, share per survivor, dust left in the DAO).
pub fn split(total: u128, fee_bps: u16, survivors: u128) -> (u128, u128, u128) {
    let fee = total * fee_bps as u128 / 10_000;
    let pool = total - fee;
    if survivors == 0 {
        return (fee, 0, pool);
    }
    let share = pool / survivors;
    (fee, share, pool - share * survivors)
}

fn transfer_kind(to: &AccountId, amount: u128) -> Value {
    json!({"Transfer": {"token_id": "", "receiver_id": to, "amount": U128(amount), "msg": null}})
}

#[near]
impl StakeStreak {
    #[init]
    pub fn new(factory: AccountId) -> Self {
        Self {
            factory,
            pots: IterableMap::new(b"p"),
        }
    }

    /// Create a pot: a Sputnik DAO named `<name>.<factory>` whose policy lets this contract, and
    /// nobody else, file and approve Transfer proposals. The attached deposit minus a small
    /// storage reserve goes to the DAO account and stays there (it pays the DAO's storage).
    #[payable]
    pub fn create_pot(&mut self, args: CreatePotArgs) -> Promise {
        let creator = env::predecessor_account_id();
        let attached = env::attached_deposit();
        require!(
            attached >= STORAGE_RESERVE.saturating_add(MIN_DAO_DEPOSIT),
            "ERR_DEPOSIT_TOO_SMALL"
        );
        require!(!self.pots.contains_key(&args.name), "ERR_POT_EXISTS");
        require!(
            args.duration_days >= 2 && args.duration_days <= MAX_DAYS,
            "ERR_DURATION_2_TO_90_DAYS"
        );
        require!(args.day_secs >= 10, "ERR_DAY_TOO_SHORT");
        require!(args.fee_bps <= MAX_FEE_BPS, "ERR_FEE_OVER_3_PERCENT");
        require!(
            args.min_members >= 2 && args.min_members as usize <= MAX_MEMBERS,
            "ERR_MIN_MEMBERS_2_TO_20"
        );
        require!(args.stake.0 >= 1_000_000_000_000_000_000_000, "ERR_STAKE_UNDER_0_001_NEAR");
        require!(args.rule.len() <= 120, "ERR_RULE_TOO_LONG");
        let dao_deposit = attached.saturating_sub(STORAGE_RESERVE);

        let now = env::block_timestamp();
        let dao: AccountId = format!("{}.{}", args.name, self.factory)
            .parse()
            .unwrap_or_else(|_| env::panic_str("ERR_BAD_POT_NAME"));
        self.pots.insert(
            args.name.clone(),
            Pot {
                dao,
                creator: creator.clone(),
                fee_account: args.fee_account.clone(),
                rule: args.rule.clone(),
                fee_bps: args.fee_bps,
                stake: args.stake.0,
                freeze_fee: args.freeze_fee.0,
                duration_days: args.duration_days,
                day_ns: args.day_secs as u64 * 1_000_000_000,
                start_ns: now + args.join_secs as u64 * 1_000_000_000,
                min_members: args.min_members,
                created: false,
                members: vec![],
                payouts: vec![],
                settled: false,
            },
        );

        let dao_args = json!({
            "config": {"name": args.name, "purpose": args.rule, "metadata": ""},
            "policy": {
                "roles": [{
                    "name": "settler",
                    "kind": {"Group": [env::current_account_id()]},
                    "permissions": ["transfer:AddProposal", "transfer:VoteApprove", "transfer:Finalize"],
                    "vote_policy": {"transfer": {"weight_kind": "RoleWeight", "quorum": "0", "threshold": "1"}}
                }],
                "default_vote_policy": {"weight_kind": "RoleWeight", "quorum": "0", "threshold": "1"},
                "proposal_bond": "0",
                "proposal_period": PROPOSAL_PERIOD_NS,
                "bounty_bond": "0",
                "bounty_forgiveness_period": "0"
            }
        });
        let create_args = json!({
            "name": args.name,
            "args": Base64VecU8::from(serde_json::to_vec(&dao_args).unwrap()),
        });
        Promise::new(self.factory.clone())
            .function_call(
                "create".to_string(),
                serde_json::to_vec(&create_args).unwrap(),
                dao_deposit,
                GAS_FACTORY_CREATE,
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_ON_CREATED)
                    .on_dao_created(args.name, creator, U128(dao_deposit.as_yoctonear())),
            )
    }

    #[private]
    pub fn on_dao_created(&mut self, name: String, creator: AccountId, deposit: U128) -> bool {
        if near_sdk::is_promise_success() {
            self.pots.get_mut(&name).expect("ERR_NO_POT").created = true;
            true
        } else {
            self.pots.remove(&name);
            // The factory also refunds this deposit to us; pay the creator from our balance now.
            Promise::new(creator)
                .transfer(NearToken::from_yoctonear(deposit.0))
                .detach();
            false
        }
    }

    /// Join with exactly the stake. The stake is forwarded to the DAO treasury at once.
    #[payable]
    pub fn join(&mut self, pot_id: String) -> Promise {
        let who = env::predecessor_account_id();
        let now = env::block_timestamp();
        let attached = env::attached_deposit();
        let pot = self.pots.get_mut(&pot_id).unwrap_or_else(|| env::panic_str("ERR_NO_POT"));
        require!(pot.created, "ERR_POT_NOT_READY");
        require!(phase_of(pot, now) == Phase::Open, "ERR_JOIN_CLOSED");
        require!(!pot.members.iter().any(|m| m.account == who), "ERR_ALREADY_MEMBER");
        require!(pot.members.len() < MAX_MEMBERS, "ERR_POT_FULL");
        require!(attached.as_yoctonear() == pot.stake, "ERR_STAKE_MUST_BE_EXACT");
        pot.members.push(Member {
            account: who.clone(),
            days: 0,
            freezes: 0,
            funded: false,
        });
        Promise::new(pot.dao.clone())
            .transfer(attached)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_ON_STAKE)
                    .on_stake(pot_id, who, U128(attached.as_yoctonear())),
            )
    }

    #[private]
    pub fn on_stake(&mut self, pot_id: String, who: AccountId, amount: U128) {
        let pot = self.pots.get_mut(&pot_id).expect("ERR_NO_POT");
        if near_sdk::is_promise_success() {
            if let Some(m) = pot.members.iter_mut().find(|m| m.account == who) {
                m.funded = true;
            }
        } else {
            pot.members.retain(|m| m.account != who);
            Promise::new(who).transfer(NearToken::from_yoctonear(amount.0)).detach();
        }
    }

    /// Leave before the pot starts. The refund is a Transfer proposal approved by the policy role.
    pub fn leave(&mut self, pot_id: String) {
        let who = env::predecessor_account_id();
        let now = env::block_timestamp();
        let pot = self.pots.get_mut(&pot_id).unwrap_or_else(|| env::panic_str("ERR_NO_POT"));
        require!(phase_of(pot, now) == Phase::Open, "ERR_ALREADY_STARTED");
        let pos = pot
            .members
            .iter()
            .position(|m| m.account == who)
            .unwrap_or_else(|| env::panic_str("ERR_NOT_MEMBER"));
        require!(pot.members[pos].funded, "ERR_STAKE_NOT_IN_DAO_YET");
        pot.members.remove(pos);
        let stake = pot.stake;
        pot.payouts.push(Payout {
            to: who,
            amount: U128(stake),
            kind: PayKind::Refund,
            state: PayState::Unpaid,
        });
        let index = (pot.payouts.len() - 1) as u32;
        self.start_pay(&pot_id, index).detach();
    }

    /// The daily check-in. No deposit, so a function-call access key limited to this method works.
    pub fn check_in(&mut self, pot_id: String) -> u32 {
        let who = env::predecessor_account_id();
        let now = env::block_timestamp();
        let pot = self.pots.get_mut(&pot_id).unwrap_or_else(|| env::panic_str("ERR_NO_POT"));
        let day = match phase_of(pot, now) {
            Phase::Running(d) => d,
            Phase::Open => env::panic_str("ERR_NOT_STARTED"),
            Phase::Ended => env::panic_str("ERR_POT_ENDED"),
        };
        let m = pot
            .members
            .iter_mut()
            .find(|m| m.account == who)
            .unwrap_or_else(|| env::panic_str("ERR_NOT_MEMBER"));
        require!(m.funded, "ERR_STAKE_NOT_IN_DAO");
        m.days |= 1u128 << day;
        env::log_str(&format!("checkin pot={} who={} day={}", pot_id, who, day));
        day
    }

    /// Buy a streak freeze: covers yesterday if it was missed. At most two per pot. Needs a
    /// wallet signature (a deposit), so it cannot be done with the check-in key.
    #[payable]
    pub fn use_freeze(&mut self, pot_id: String) -> Promise {
        let who = env::predecessor_account_id();
        let now = env::block_timestamp();
        let attached = env::attached_deposit();
        let pot = self.pots.get_mut(&pot_id).unwrap_or_else(|| env::panic_str("ERR_NO_POT"));
        let day = match phase_of(pot, now) {
            Phase::Running(d) if d >= 1 => d,
            _ => env::panic_str("ERR_NOTHING_TO_FREEZE"),
        };
        require!(attached.as_yoctonear() == pot.freeze_fee, "ERR_FREEZE_FEE_MUST_BE_EXACT");
        let fee_account = pot.fee_account.clone();
        let m = pot
            .members
            .iter_mut()
            .find(|m| m.account == who)
            .unwrap_or_else(|| env::panic_str("ERR_NOT_MEMBER"));
        require!(m.freezes < MAX_FREEZES, "ERR_MAX_2_FREEZES");
        require!(m.days & (1u128 << (day - 1)) == 0, "ERR_YESTERDAY_NOT_MISSED");
        m.days |= 1u128 << (day - 1);
        m.freezes += 1;
        Promise::new(fee_account).transfer(attached)
    }

    /// Anyone can settle once the pot has ended (or once it started without enough members).
    /// Fixes the payout list from the contract's own books, then files and approves up to `batch`
    /// of the Transfer proposals. Call again until `payouts_pending` is 0.
    pub fn settle(&mut self, pot_id: String, batch: Option<u32>) -> u32 {
        let now = env::block_timestamp();
        let pot = self.pots.get_mut(&pot_id).unwrap_or_else(|| env::panic_str("ERR_NO_POT"));
        require!(pot.created, "ERR_POT_NOT_READY");
        let phase = phase_of(pot, now);
        let enough = pot.members.len() as u32 >= pot.min_members;
        require!(
            phase == Phase::Ended || (phase != Phase::Open && !enough),
            "ERR_NOT_ENDED"
        );
        if !pot.settled {
            let funded: Vec<Member> = pot.members.iter().filter(|m| m.funded).cloned().collect();
            if !enough {
                for m in &funded {
                    pot.payouts.push(Payout {
                        to: m.account.clone(),
                        amount: U128(pot.stake),
                        kind: PayKind::Refund,
                        state: PayState::Unpaid,
                    });
                }
            } else {
                let completed = pot.duration_days;
                let survivors: Vec<&Member> =
                    funded.iter().filter(|m| alive(m.days, completed)).collect();
                let total = pot.stake * funded.len() as u128;
                let (fee, share, _dust) = split(total, pot.fee_bps, survivors.len() as u128);
                if survivors.is_empty() {
                    // Same as the Solana product: nobody survived, the pot goes to the fee account.
                    pot.payouts.push(Payout {
                        to: pot.fee_account.clone(),
                        amount: U128(total),
                        kind: PayKind::Fee,
                        state: PayState::Unpaid,
                    });
                } else {
                    if fee > 0 {
                        pot.payouts.push(Payout {
                            to: pot.fee_account.clone(),
                            amount: U128(fee),
                            kind: PayKind::Fee,
                            state: PayState::Unpaid,
                        });
                    }
                    for m in survivors {
                        pot.payouts.push(Payout {
                            to: m.account.clone(),
                            amount: U128(share),
                            kind: PayKind::Win,
                            state: PayState::Unpaid,
                        });
                    }
                }
            }
            pot.settled = true;
        }
        let batch = batch.unwrap_or(3).max(1);
        require!(
            env::prepaid_gas() >= GAS_PER_PAYOUT.saturating_mul(batch as u64).saturating_add(Gas::from_tgas(10)),
            "ERR_ATTACH_MORE_GAS_FOR_BATCH"
        );
        let todo: Vec<u32> = pot
            .payouts
            .iter()
            .enumerate()
            .filter(|(_, p)| !matches!(p.state, PayState::Approved(_)))
            .map(|(i, _)| i as u32)
            .take(batch as usize)
            .collect();
        for i in &todo {
            self.start_pay(&pot_id, *i).detach();
        }
        self.pots
            .get(&pot_id)
            .map(|p| pending(p))
            .unwrap_or(0)
    }

    /// Push one payout along (anyone). Safe to repeat: a payout already approved is a no-op.
    pub fn pay(&mut self, pot_id: String, index: u32) -> PromiseOrValue<()> {
        let pot = self.pots.get(&pot_id).unwrap_or_else(|| env::panic_str("ERR_NO_POT"));
        require!((index as usize) < pot.payouts.len(), "ERR_NO_PAYOUT");
        if matches!(pot.payouts[index as usize].state, PayState::Approved(_)) {
            return PromiseOrValue::Value(());
        }
        self.start_pay(&pot_id, index).into()
    }

    #[private]
    pub fn on_filed(&mut self, pot_id: String, index: u32) -> PromiseOrValue<()> {
        let id: u64 = match env::promise_result_checked(0, 64) {
            Ok(bytes) => match serde_json::from_slice(&bytes) {
                Ok(id) => id,
                Err(_) => return PromiseOrValue::Value(()),
            },
            Err(_) => return PromiseOrValue::Value(()),
        };
        let pot = self.pots.get_mut(&pot_id).expect("ERR_NO_POT");
        let p = &mut pot.payouts[index as usize];
        // Only the first filing counts; a concurrent second filing stays unapproved forever.
        if p.state != PayState::Unpaid {
            return PromiseOrValue::Value(());
        }
        p.state = PayState::Filed(id);
        let (dao, to, amount) = (pot.dao.clone(), p.to.clone(), p.amount.0);
        self.approve(dao, pot_id, index, id, &to, amount).into()
    }

    #[private]
    pub fn on_approved(&mut self, pot_id: String, index: u32, proposal_id: u64) {
        if !near_sdk::is_promise_success() {
            return;
        }
        let pot = self.pots.get_mut(&pot_id).expect("ERR_NO_POT");
        let p = &mut pot.payouts[index as usize];
        if p.state == PayState::Filed(proposal_id) {
            p.state = PayState::Approved(proposal_id);
        }
    }

    // ---- views ----

    pub fn get_pot(&self, pot_id: String) -> Option<PotView> {
        self.pots.get(&pot_id).map(|p| view(&pot_id, p))
    }

    pub fn list_pots(&self, from: u32, limit: u32) -> Vec<PotView> {
        self.pots
            .iter()
            .skip(from as usize)
            .take(limit as usize)
            .map(|(k, p)| view(k, p))
            .collect()
    }

    pub fn get_factory(&self) -> AccountId {
        self.factory.clone()
    }
}

impl StakeStreak {
    /// File the Transfer proposal, or approve it if it is already filed.
    fn start_pay(&self, pot_id: &String, index: u32) -> Promise {
        let pot = self.pots.get(pot_id).expect("ERR_NO_POT");
        let p = &pot.payouts[index as usize];
        match p.state {
            PayState::Unpaid => Promise::new(pot.dao.clone())
                .function_call(
                    "add_proposal".to_string(),
                    serde_json::to_vec(&json!({"proposal": {
                        "description": format!("stakestreak {:?} pot={}", p.kind, pot_id),
                        "kind": transfer_kind(&p.to, p.amount.0),
                    }}))
                    .unwrap(),
                    NearToken::from_yoctonear(0),
                    GAS_ADD_PROPOSAL,
                )
                .then(
                    Self::ext(env::current_account_id())
                        .with_static_gas(GAS_PER_PAYOUT.saturating_sub(GAS_ADD_PROPOSAL))
                        .on_filed(pot_id.clone(), index),
                ),
            PayState::Filed(id) => self.approve(pot.dao.clone(), pot_id.clone(), index, id, &p.to, p.amount.0),
            PayState::Approved(_) => env::panic_str("ERR_ALREADY_APPROVED"),
        }
    }

    fn approve(&self, dao: AccountId, pot_id: String, index: u32, id: u64, to: &AccountId, amount: u128) -> Promise {
        Promise::new(dao)
            .function_call(
                "act_proposal".to_string(),
                serde_json::to_vec(&json!({
                    "id": id,
                    "action": "VoteApprove",
                    "proposal": transfer_kind(to, amount),
                    "memo": null,
                }))
                .unwrap(),
                NearToken::from_yoctonear(0),
                GAS_ACT_PROPOSAL,
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_ON_APPROVED)
                    .on_approved(pot_id, index, id),
            )
    }
}

fn pending(p: &Pot) -> u32 {
    p.payouts
        .iter()
        .filter(|x| !matches!(x.state, PayState::Approved(_)))
        .count() as u32
}

fn view(name: &str, p: &Pot) -> PotView {
    let now = env::block_timestamp();
    let phase = phase_of(p, now);
    let completed = completed_days(p, now);
    PotView {
        name: name.to_string(),
        dao: p.dao.clone(),
        creator: p.creator.clone(),
        fee_account: p.fee_account.clone(),
        rule: p.rule.clone(),
        fee_bps: p.fee_bps,
        stake: U128(p.stake),
        freeze_fee: U128(p.freeze_fee),
        duration_days: p.duration_days,
        day_ns: p.day_ns,
        start_ns: p.start_ns,
        min_members: p.min_members,
        created: p.created,
        phase: if !p.created {
            "creating"
        } else if p.settled {
            "settled"
        } else {
            match phase {
                Phase::Open => "open",
                Phase::Running(_) => "running",
                Phase::Ended => "ended",
            }
        }
        .to_string(),
        day: match phase {
            Phase::Running(d) => Some(d),
            _ => None,
        },
        members: p
            .members
            .iter()
            .map(|m| MemberView {
                account: m.account.clone(),
                funded: m.funded,
                alive: alive(m.days, completed),
                checkins: m.days.count_ones(),
                freezes: m.freezes,
                days: U128(m.days),
            })
            .collect(),
        payouts: p.payouts.clone(),
        payouts_pending: pending(p),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alive_needs_every_completed_day() {
        assert!(alive(0, 0));
        assert!(alive(0b1, 1));
        assert!(!alive(0b0, 1));
        assert!(alive(0b111, 3));
        // day 1 missed, later days present: still out
        assert!(!alive(0b101, 3));
        // extra bit beyond the completed days does not matter
        assert!(alive(0b1011, 2));
    }

    #[test]
    fn split_is_equal_and_dust_stays() {
        // 4 x 10 NEAR, 3% fee, 3 survivors
        let ten = 10_000_000_000_000_000_000_000_000u128;
        let (fee, share, dust) = split(4 * ten, 300, 3);
        assert_eq!(fee, 4 * ten * 3 / 100);
        assert_eq!(share * 3 + dust + fee, 4 * ten);
        assert!(dust < 3);
        let (fee0, share0, dust0) = split(4 * ten, 0, 4);
        assert_eq!((fee0, share0, dust0), (0, ten, 0));
    }
}
