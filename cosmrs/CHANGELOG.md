# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.10.0 (2022-11-07)
### Added
- `Coin` constructor ([#291])
- Impl `From<AccountId>` for `String` ([#301])

### Changed
- Bump tendermint-rs crates to v0.26; MSRV 1.59 ([#302])
- Refactor into submodules ([#303])
- Bump `cosmos-sdk-proto` to v0.15 ([#304])

[#291]: https://github.com/cosmos/cosmos-rust/pull/291
[#301]: https://github.com/cosmos/cosmos-rust/pull/301
[#302]: https://github.com/cosmos/cosmos-rust/pull/302
[#303]: https://github.com/cosmos/cosmos-rust/pull/303
[#304]: https://github.com/cosmos/cosmos-rust/pull/304

## 0.9.0 (2022-08-08)
### Added
- `tx::BodyBuilder` ([#254])
- Additional `feegrant` domain types ([#280])

### Changed
- Allow alphanumeric Bech32 prefixes on `AccountId` ([#281])
- Bump tendermint-rs crates to v0.23.9 ([#277])
- Bump `cosmos-sdk-proto` to v0.14 ([#283])

### Removed
- Direct dependencies on `prost` ([#282])

[#254]: https://github.com/cosmos/cosmos-rust/pull/254
[#277]: https://github.com/cosmos/cosmos-rust/pull/277
[#280]: https://github.com/cosmos/cosmos-rust/pull/280
[#281]: https://github.com/cosmos/cosmos-rust/pull/281
[#282]: https://github.com/cosmos/cosmos-rust/pull/282
[#283]: https://github.com/cosmos/cosmos-rust/pull/283


## 0.8.0 (2022-07-25)
### Added
- `feegrant` module support ([#250])
- `grpc` features ([#258])

### Changed
- Changed internal `Coin` representation to `u128` ([#235])
- Implemented `serde` traits for `Denom` ([#235])
- Bump `k256` to v0.11 ([#253])
- Bump tendermint-rs crates to v0.23.8 ([#253])
- Move protobuf traits from `cosmrs` => `cosmos-sdk-proto` ([#255])
- MSRV 1.57 ([#257])
- Bump `cosmos-sdk-proto` to v0.13.0 ([#260])

### Fixed
- Visibility on `cosmwasm::ContractInfo` ([#247])

### Removed
- `Decimal` type ([#235])

[#235]: https://github.com/cosmos/cosmos-rust/pull/235
[#247]: https://github.com/cosmos/cosmos-rust/pull/247
[#250]: https://github.com/cosmos/cosmos-rust/pull/250
[#253]: https://github.com/cosmos/cosmos-rust/pull/253
[#255]: https://github.com/cosmos/cosmos-rust/pull/255
[#257]: https://github.com/cosmos/cosmos-rust/pull/257
[#258]: https://github.com/cosmos/cosmos-rust/pull/258
[#260]: https://github.com/cosmos/cosmos-rust/pull/260


## 0.7.1 (2022-06-09)
### Added
- `abci`, `auth`, `cosmwasm`, and `vesting` type wrappers ([#234])
- `bank::MsgMultiSend` support ([#237])

### Fixed
- Remove unneeded generic type parameter to `SigningKey::derive_from_path` ([#243])

[#234]: https://github.com/cosmos/cosmos-rust/pull/234
[#237]: https://github.com/cosmos/cosmos-rust/pull/237
[#243]: https://github.com/cosmos/cosmos-rust/pull/243


## 0.7.0 (2022-05-02)
### Changed
- Bump tendermint-rs crates to v0.23.7 ([#215])
- Bump `prost` to v0.10 ([#215])
- Bump `tonic` to v0.7 ([#215])
- Bump `cosmos-sdk-proto` to v0.12.0 ([#217])

[#215]: https://github.com/cosmos/cosmos-rust/pull/215
[#217]: https://github.com/cosmos/cosmos-rust/pull/217


## 0.6.1 (2022-04-28)
### Fixed
- Better error message on `AccountId` Bech32 decode failure ([#209])
- `tx::Fee` parsing with empty address fields ([#210])

[#209]: https://github.com/cosmos/cosmos-rust/pull/209
[#210]: https://github.com/cosmos/cosmos-rust/pull/210


## 0.6.0 (2022-04-22)
### Added
- `TryFrom` impls for `tendermint` public key types ([#203])

### Changed
- Support variable-width `AccountId` using new Cosmos SDK rules ([#197], [#204])
- Bump tendermint-rs crates to v0.23.6 ([#205])
- Bump `cosmos-sdk-proto` to v0.11.0 ([#206])

[#197]: https://github.com/cosmos/cosmos-rust/pull/197
[#203]: https://github.com/cosmos/cosmos-rust/pull/203
[#204]: https://github.com/cosmos/cosmos-rust/pull/204
[#205]: https://github.com/cosmos/cosmos-rust/pull/205
[#206]: https://github.com/cosmos/cosmos-rust/pull/206


## 0.5.1 (2022-03-14)
### Fixed
- `Denom` parsing for IBC addresses ([#182])

[#182]: https://github.com/cosmos/cosmos-rust/pull/182


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
