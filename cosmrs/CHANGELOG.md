# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.0 (2022-03-10)
### Changed
- Bump `tendermint-proto` to v0.23.5 ([#178])
- Bump `cosmos-sdk-proto` to v0.10.0 ([#180])

### Removed
- `Option` from field amount of `MsgUndelegate` and `MsgBeginRedelegate` ([#175])

[#175]: https://github.com/cosmos/cosmos-rust/pull/175
[#178]: https://github.com/cosmos/cosmos-rust/pull/178
[#180]: https://github.com/cosmos/cosmos-rust/pull/180

## 0.4.1 (2022-01-10)
### Changed
- Bump `cosmos-sdk-proto` to v0.4.1 ([#165])

[#165]: https://github.com/cosmos/cosmos-rust/pull/165

## 0.4.0 (2022-01-07) [YANKED]
### Changed
- Bump `tendermint` to v0.23.3; `k256` to v0.10 ([#163])
- Use `Vec<u8>` as the `Tx::signatures` type ([#164])

[#163]: https://github.com/cosmos/cosmos-rust/pull/163
[#164]: https://github.com/cosmos/cosmos-rust/pull/164

## 0.3.0 (2021-10-28)
### Added
- Associated `Proto` type to `Msg` trait - formerly named `MsgType` ([#146])
- `from_any`/`to_any` methods to `Msg` trait - formerly named `MsgType` ([#146])
- `/cosmos.crypto.multisig.LegacyAminoPubKey` support ([#147])
- `SignerPublicKey` enum for `SignerInfo::public_key` ([#147])

### Changed
- Bump tendermint-rs crates to v0.23 ([#144])
- Bump prost crates to v0.9 ([#144])
- Bump tonic to v0.6 ([#144])
- MSRV 1.56 ([#144])
- Renamed `tx::MsgType` trait to `tx::Msg` ([#146])
- Renamed `MsgProto::from_msg`/`to_msg` to `from_any`/`to_any`([#146])
- Bump `cosmos-sdk-proto` to v0.8 ([#149])

### Removed
- `tx::Msg` newtype for `prost_types::Any`. Use `Any` instead ([#146])

[#144]: https://github.com/cosmos/cosmos-rust/pull/144
[#146]: https://github.com/cosmos/cosmos-rust/pull/146
[#147]: https://github.com/cosmos/cosmos-rust/pull/147
[#149]: https://github.com/cosmos/cosmos-rust/pull/149

## 0.2.1 (2021-10-06)
### Added
- `PublicKey` JSON serialization support ([#133])

[#133]: https://github.com/cosmos/cosmos-rust/pull/133

## 0.2.0 (2021-09-27)
### Changed
- Make `Tx::find_by_hash` use the `/tx` endpoint ([#116])
- Update `tendermint` crate to v0.22 ([#128])
- Update `cosmos-sdk-proto` to v0.7 ([#129])

[#116]: https://github.com/cosmos/cosmos-rust/pull/116
[#128]: https://github.com/cosmos/cosmos-rust/pull/128
[#129]: https://github.com/cosmos/cosmos-rust/pull/129

## 0.1.0 (2021-08-25)
- Initial release under `cosmrs` name
