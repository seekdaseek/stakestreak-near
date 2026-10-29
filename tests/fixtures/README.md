# Sputnik fixtures

`factory.wasm` is the code of `sputnikv2.testnet` and `dao.wasm` is the default DAO code it serves
(code hash `AA4nA45vauiac3jdVwJTzSdUP3oj2r1fDaPAb2mHdgcH`, version 3.31), fetched on 2026-10-29
with `scripts/fetch-sputnik.sh`. The sandbox tests deploy exactly these bytes, so they exercise the
Sputnik that is live on testnet and not a copy built from source. Sputnik is MIT licensed
(github.com/near-daos/sputnik-dao-contract). The upstream source at its `main` does not match the
deployed factory in one respect we measured: the deployed `create` demands 5.3968 NEAR (the DAO
code is stored in the factory), where the source on `main` asks for 0.01 NEAR.
