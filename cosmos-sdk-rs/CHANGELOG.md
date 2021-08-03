# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.0 (2021-08-02)
### Added
- Basic `cosmwasm` support ([#96])
- Distribution module ([#114])

### Changed
- Update protobufs for cosmos-sdk 0.43.0-rc2 ([#108])
- Bump `tendermint-rs` to v0.21 ([#110])
- Bump `cosmos-sdk-proto` to v0.6 ([#111])
- Made admin field optional in `MsgInstantiateContract` ([#115])

[#96]: https://github.com/cosmos/cosmos-rust/pull/96
[#108]: https://github.com/cosmos/cosmos-rust/pull/108
[#110]: https://github.com/cosmos/cosmos-rust/pull/110
[#111]: https://github.com/cosmos/cosmos-rust/pull/111
[#114]: https://github.com/cosmos/cosmos-rust/pull/114
[#115]: https://github.com/cosmos/cosmos-rust/pull/115

## 0.2.0 (2021-06-24)
### Added
- Staking functions ([#82])
- More detail to `AccountId` parsing errors ([#85])
- Basic BIP32 support ([#91])

### Changed
- Bump `tendermint*` crate dependencies to v0.20; MSRV 1.51+ ([#89])

[#82]: https://github.com/cosmos/cosmos-rust/pull/82
[#85]: https://github.com/cosmos/cosmos-rust/pull/85
[#89]: https://github.com/cosmos/cosmos-rust/pull/89
[#91]: https://github.com/cosmos/cosmos-rust/pull/91

## 0.1.1 (2021-04-13)
- Initial release
