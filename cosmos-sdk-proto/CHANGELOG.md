# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.12.3 (2022-06-09)
### Added
- Additional `cosmwasm` protos ([#240], [#244])

[#240]: https://github.com/cosmos/cosmos-rust/pull/240
[#244]: https://github.com/cosmos/cosmos-rust/pull/244

## 0.12.2 (2022-05-16)
### Added
- `grpc-transport` crate feature ([#230])

[#230]: https://github.com/cosmos/cosmos-rust/pull/230

## 0.12.1 (2022-05-12)
### Fixed
- Clashing protobuf namespaces between `cosmos-sdk` and `ibc` ([#220])

[#220]: https://github.com/cosmos/cosmos-rust/pull/220

## 0.12.0 (2022-05-02) [YANKED]

NOTE: this release was yanked due to the clashing namespace bug ([#220])

### Changed
- Bump tendermint-rs crates to v0.23.7 ([#215])
- Bump `prost` to v0.10 ([#215])
- Bump `tonic` to v0.7 ([#215])
- Bump `COSMOS_SDK_REV` from v0.45.2 => v0.45.4 ([#215])

[#215]: https://github.com/cosmos/cosmos-rust/pull/215

## 0.11.0 (2022-04-21)
### Added
- Missing modules from `ibc-go` ([#187])

### Changed
- Upgrade SDK version => v0.45.2, IBC => v3.0.0 ([#199])
- Bump tendermint-rs crates to v0.23.6 ([#205])

### Fixed
- Export `authz` module ([#186])
- Export `feegrant` module ([#198])

[#186]: https://github.com/cosmos/cosmos-rust/pull/186
[#187]: https://github.com/cosmos/cosmos-rust/pull/187
[#198]: https://github.com/cosmos/cosmos-rust/pull/198
[#199]: https://github.com/cosmos/cosmos-rust/pull/199
[#205]: https://github.com/cosmos/cosmos-rust/pull/205

## 0.10.0 (2022-03-10)
### Added
- `authz` and `feegrant` protos ([#177])

### Changed
- Update SDK version => v0.45.1, IBC => v2.0.3, wasmd => v0.23.0 ([#177])
- Bump `tendermint-proto` to v0.23.5 ([#178])

[#177]: https://github.com/cosmos/cosmos-rust/pull/177
[#178]: https://github.com/cosmos/cosmos-rust/pull/178

## 0.9.0 (2022-01-10)
### Changed
- Update wasmd compatibility to 0.21 ([#158])
- Bump `tendermint-proto` to v0.23.3 ([#163])

[#158]: https://github.com/cosmos/cosmos-rust/pull/158
[#163]: https://github.com/cosmos/cosmos-rust/pull/163

## 0.8.0 (2021-10-28)
### Changed
- Update to cosmos-sdk v0.44.1 and ibc-go v1.2.0 ([#138])
- Bump tendermint-rs crates to v0.23; MSRV 1.56 ([#144])
- 2021 edition upgrade ([#148])

[#138]: https://github.com/cosmos/cosmos-rust/pull/138
[#144]: https://github.com/cosmos/cosmos-rust/pull/144
[#148]: https://github.com/cosmos/cosmos-rust/pull/148

## 0.7.0 (2021-09-27)
### Changed
- Update `tendermint-proto` crate to v0.22 ([#128])
- Bump `COSMOS_REV` to v0.44.0 ([#130])

[#128]: https://github.com/cosmos/cosmos-rust/pull/128
[#130]: https://github.com/cosmos/cosmos-rust/pull/130

## 0.6.3 (2021-08-24)
### Changed
- Bump MSRV to 1.54 ([#122])

[#122]: https://github.com/cosmos/cosmos-rust/pull/122

## 0.6.2 (2021-08-24)
### Added
- Protos for vesting accounts ([#119])

[#119]: https://github.com/cosmos/cosmos-rust/pull/119

## 0.6.1 (2021-08-19)
### Changed
- Rebuild protos with cosmos-sdk v0.43.0  ([#117])

[#117]: https://github.com/cosmos/cosmos-rust/pull/117

## 0.6.0 (2021-08-02) [YANKED]
### Added
- Basic `cosmwasm` support ([#96])

### Changed
- Bump `tendermint-proto` requirement from 0.20 to 0.21 ([#106])

[#96]: https://github.com/cosmos/cosmos-rust/pull/96
[#106]: https://github.com/cosmos/cosmos-rust/pull/106

## 0.5.0 (2021-04-10)
### Changed
- Add support for crypto proto and services ([#73])
- Update `tendermint` crate ([#72])

[#72]: https://github.com/cosmos/cosmos-rust/pull/72
[#73]: https://github.com/cosmos/cosmos-rust/pull/73

## 0.4.0 (2021-04-02)
### Changed
- Add support for bank proto and services ([#61])
- Add support for tendermint proto and services ([#57])
- Add support for bank, crisis, distribution, evidence, genutil, gov, mint, params, slashing, staking upgrade and vesting proto and services ([#64])
- Bump COSMOS_REV to v0.42.3 ([#57])

[#57]: https://github.com/cosmos/cosmos-rust/pull/57
[#61]: https://github.com/cosmos/cosmos-rust/pull/61
[#61]: https://github.com/cosmos/cosmos-rust/pull/64

## 0.3.0 (2020-02-01)
### Changed
- Bump `cosmos-sdk` rev to v0.40.0 ([#37])
- Bump `tendermint` crate dependency to v0.18 ([#45])
- Bump `prost`, `prost-types`, `prost-build` to v0.7 ([#45])
- Bump `tonic`, `tonic-build` to v0.4 ([#45])

[#37]: https://github.com/cosmos/cosmos-rust/pull/37
[#45]: https://github.com/cosmos/cosmos-rust/pull/45
[#45]: https://github.com/cosmos/cosmos-rust/pull/45
[#45]: https://github.com/cosmos/cosmos-rust/pull/45

## 0.2.0 (2020-01-04)
### Added
- `grpc` crate feature ([#8])

### Changed
- Bump `cosmos-sdk` rev to v0.40.0-rc6 ([#32])
- Bump `tendermint` + `tendermint-proto` crate dependencies to v0.17 ([#18])
- Format `prost`/`tonic` output with `rustfmt` ([#17])

[#8]: https://github.com/cosmos/cosmos-rust/pull/8
[#17]: https://github.com/cosmos/cosmos-rust/pull/17
[#18]: https://github.com/cosmos/cosmos-rust/pull/18
[#32]: https://github.com/cosmos/cosmos-rust/pull/32

## 0.1.2 (2020-11-30)

## 0.1.1 (2020-11-30)

- Initial release
