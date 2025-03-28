# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.22.0 (2025-03-27)
### Changed
- Use `Vec::with_capacity` for `MsgFundCommunityPool` ([#511])
- Bump `cosmos-sdk-proto` to v0.27 ([#521])

[#511]: https://github.com/cosmos/cosmos-rust/pull/511
[#521]: https://github.com/cosmos/cosmos-rust/pull/521

## 0.21.1 (2025-02-06)
### Fixed
- `BodyBuilder` non-critical extension add ([#516])

[#516]: https://github.com/cosmos/cosmos-rust/pull/516

## 0.21.0 (2024-10-24)
### Changed
- Bump tendermint-rs to v0.40 ([#506])
- Bump `cosmos-sdk-proto` to v0.26 ([#508])

[#506]: https://github.com/cosmos/cosmos-rust/pull/506
[#508]: https://github.com/cosmos/cosmos-rust/pull/508

## 0.20.0 (2024-09-09)
### Changed
- Bump `cosmos-sdk-proto` to v0.25 ([#503])

[#503]: https://github.com/cosmos/cosmos-rust/pull/503

## 0.19.0 (2024-08-14)
### Changed
- Bump tendermint-rs dependencies to v0.39 ([#482], [#483])
- Bump `cosmos-sdk-proto` to v0.24 ([#493])

[#482]: https://github.com/cosmos/cosmos-rust/pull/482
[#483]: https://github.com/cosmos/cosmos-rust/pull/483
[#493]: https://github.com/cosmos/cosmos-rust/pull/493

## 0.18.0 (2024-08-02)
### Added
- Support `Coin` with amount `0` and empty denom ([#479])
- Impl `Default` for `Coin` and `Denom` ([#479])

### Changed
- Bump tendermint-rs dependencies to v0.38 ([#476])
- Bump `cosmos-sdk-proto` to 0.23 ([#480])

[#476]: https://github.com/cosmos/cosmos-rust/pull/476
[#479]: https://github.com/cosmos/cosmos-rust/pull/479
[#480]: https://github.com/cosmos/cosmos-rust/pull/480

## 0.17.0 (2024-06-27)
### Added
- Basic parsing of `msg_responses` field inside `TxMsgData` ([#472])

### Changed
- Bump tendermint-rs dependencies to v0.37 ([#469])
- Match upstream Cosmos SDK's `Denom` validation logic ([#468], [#470])
- Bump `cosmos-sdk-proto` to v0.22 ([#473])

[#468]: https://github.com/cosmos/cosmos-rust/pull/468
[#469]: https://github.com/cosmos/cosmos-rust/pull/469
[#470]: https://github.com/cosmos/cosmos-rust/pull/470
[#472]: https://github.com/cosmos/cosmos-rust/pull/472
[#473]: https://github.com/cosmos/cosmos-rust/pull/473

## 0.16.0 (2024-03-15)
### Added
- Support for uppercase Bech32 ([#444])
- Support for `slashing` module ([#452])
- Support for validator-related queries in `staking` module ([#453])
- Expose `base::query` module and pagination types ([#454])

### Changed
- Bump `tendermint`/`tendermint-rpc` dependencies to v0.35 ([#461])
- Bump `cosmos-sdk-proto` to v0.21.0 ([#463])

### Fixed
- Correctly populate `updated` and `msg` fields from proto responses ([#451])

[#444]: https://github.com/cosmos/cosmos-rust/pull/444
[#451]: https://github.com/cosmos/cosmos-rust/pull/451
[#452]: https://github.com/cosmos/cosmos-rust/pull/452
[#453]: https://github.com/cosmos/cosmos-rust/pull/453
[#454]: https://github.com/cosmos/cosmos-rust/pull/454
[#461]: https://github.com/cosmos/cosmos-rust/pull/461
[#463]: https://github.com/cosmos/cosmos-rust/pull/463

## 0.15.0 (2023-10-03)
### Added
- Export `msg_clear_admin` and `msg_update_admin` types ([#419])
- `getrandom` feature ([#434])

### Changed
- Bound `EcdsaSigner` trait on `Sync + Send` ([#410])
- MSRV 1.72 ([#428])
- Bump `tendermint`/`tendermint-rpc` dependencies to v0.34 ([#431])
- Replace `TypeUrl` with `Name` trait ([#432])
- Bump `cosmos-sdk-proto` to v0.20 ([#440])

### Fixed
- `PublicKey::to_any` ([#406])

[#406]: https://github.com/cosmos/cosmos-rust/pull/406
[#410]: https://github.com/cosmos/cosmos-rust/pull/410
[#419]: https://github.com/cosmos/cosmos-rust/pull/419
[#428]: https://github.com/cosmos/cosmos-rust/pull/428
[#431]: https://github.com/cosmos/cosmos-rust/pull/431
[#432]: https://github.com/cosmos/cosmos-rust/pull/432
[#434]: https://github.com/cosmos/cosmos-rust/pull/434
[#440]: https://github.com/cosmos/cosmos-rust/pull/440

## 0.14.0 (2023-05-03)
### Changed
- Bump `tendermint`/`tendermint-rpc` to v0.32 ([#400])
- Bump `cosmos-sdk-proto` to v0.19 ([#401])

[#400]: https://github.com/cosmos/cosmos-rust/pull/400
[#401]: https://github.com/cosmos/cosmos-rust/pull/401

## 0.13.0 (2023-04-17)
### Changed
- Bump signature + tendermint-rs dependencies; MSRV 1.65 ([#385])
  - `bip32` v0.5
  - `ecdsa` v0.16
  - `k256` v0.13
  - `tendermint` v0.31
  - `tendermint-proto` v0.31
  - `tendermint-rpc` v0.31
- Bump `cosmos-sdk-proto` to v0.18 ([#390])

[#385]: https://github.com/cosmos/cosmos-rust/pull/385
[#390]: https://github.com/cosmos/cosmos-rust/pull/390

## 0.12.0 (2023-03-23)
### Added
- Expose `cosmwasm::msg_migrate_contract` structs ([#315])

### Changed
- Bump tendermint-rs crates to 0.30 ([#354])
- MSRV 1.63 ([#356])
- Support for `wasmd` >=0.29 by generating files via `buf` ([#358])
- Bump `cosmos-sdk-proto` to v0.17 ([#367])

[#315]: https://github.com/cosmos/cosmos-rust/pull/315
[#354]: https://github.com/cosmos/cosmos-rust/pull/354
[#356]: https://github.com/cosmos/cosmos-rust/pull/356
[#358]: https://github.com/cosmos/cosmos-rust/pull/358
[#367]: https://github.com/cosmos/cosmos-rust/pull/367

## 0.11.0 (2022-11-30)
### Changed
- Bump tendermint-rs crates to 0.27 ([#306])
- Bump `cosmos-sdk-proto` to v0.16 ([#307])

[#306]: https://github.com/cosmos/cosmos-rust/pull/306
[#307]: https://github.com/cosmos/cosmos-rust/pull/307

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
