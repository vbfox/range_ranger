# Range ranger

[![License][license-badge]](LICENSE)
[![crates.io][crate-badge]][crate]
[![Docs][docs-badge]][docs]
[![GitHub Actions Status][actions-badge]][actions]
[![msrv]][releases.rs]

[license-badge]: https://img.shields.io/badge/License-MIT-green.svg?longCache=true
[crate-badge]: https://img.shields.io/crates/v/range_ranger.svg
[crate]: https://crates.io/crates/range_ranger
[docs-badge]: https://docs.rs/range_ranger/badge.svg
[docs]: https://docs.rs/range_ranger
[actions-badge]: https://github.com/vbfox/range_ranger/actions/workflows/ci.yaml/badge.svg?branch=main
[actions]: https://github.com/vbfox/range_ranger/actions/workflows/ci.yaml?query=branch%3Amain
[msrv]: https://img.shields.io/crates/msrv/range_ranger.svg
[releases.rs]: https://releases.rs/#rust-versions

**This library is a WORK IN PROGRESS experiment into rust APIs for me**

Range ranger is a range manipulation library supporting multiple type of ranges under a single type:

* Empty
* Continuous (containing all values between min and max)
  * Bounds can be inclusive or exclusive
  * Bounds can be finite or infinite
* Full ranges
* List of values
* Single value
* Composite ranges (Union of multiple of theses)

With support for the following operations:

* Union
* Intersection
* Difference
* Contains range
* Contains value
* Overlaps
* Simplification

The range type is an enum of all the possible range subtypes.
The default behaviour for ranges constructed via methods is to be simplified and sorted but non-simplified ranges can be constructed by creating the enum members directly.

## Running coverage

```powershell
# Requirements
cargo install grcov to install
rustup component add llvm-tools-preview

# Running
./coverage.ps1
```

## See also

* https://gitlab.com/bit-refined/ranges/ Another rust range library
* https://www.postgresql.org/docs/current/functions-range.html PostgreSQL operation on ranges
* https://www.postgresql.org/docs/14/rangetypes.html PostgreSQL range types
