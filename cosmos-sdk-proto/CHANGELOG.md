# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
