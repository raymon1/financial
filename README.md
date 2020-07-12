# Financial

[![CircleCI][circle-ci-image]][circle-ci-build] [![crates.io][cratesio-image]][cratesio] [![docs.rs][docsrs-image]][docsrs]

[circle-ci-image]: https://circleci.com/gh/raymon1/financial.svg?style=shield
[circle-ci-build]: https://app.circleci.com/pipelines/github/raymon1/financial
[cratesio-image]: https://img.shields.io/crates/v/financial.svg
[cratesio]: https://crates.io/crates/financial
[docsrs-image]: https://docs.rs/financial/badge.svg
[docsrs]: https://docs.rs/crate/financial

`Financial` is a [Rust](https://www.rust-lang.org/) crate that contains collection of finance calculations mimicking some of [Excel Financial Functions](https://support.microsoft.com/en-us/office/financial-functions-reference-5658d81e-6035-4f24-89c1-fbf124c2b1d8) interface.
you can find the crate [here](https://docs.rs/crate/financial)

## What is different than other financial crates?

It supports both periodic and scheduled computation for [IRR](https://en.wikipedia.org/wiki/Internal_rate_of_return) and [NPV](https://en.wikipedia.org/wiki/Net_present_value).

IRR and NPV functions should be faster since powers are pre-computed iteratively instead of using power function multiple times.

## Supported Functions

- FV(Rate, Nper, Pmt, Pv, Pmt_is_due)
- PV(Rate, Nper, Pmt, Fv, Pmt_is_due)
- NPV(Rate, values)
- XNPV(Rate), values, dates)
- IRR(values)
- XIRR(values, dates)

## Future Work

- Add More Functions
- Add bench tests
