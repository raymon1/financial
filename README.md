# Financial

 [![GitHub][gh-ci-img]][gh-ci] [![CircleCI][circle-ci-img]][circle-ci] [![crates.io][cratesio-img]][cratesio] [![docs.rs][docsrs-img]][docsrs]

[gh-ci-img]: https://github.com/raymon1/financial/workflows/Rust/badge.svg
[gh-ci]: https://github.com/raymon1/financial/actions?query=workflow%3ARust
[circle-ci-img]: https://circleci.com/gh/raymon1/financial.svg?style=shield
[circle-ci]: https://app.circleci.com/pipelines/github/raymon1/financial
[cratesio-img]: https://img.shields.io/crates/v/financial.svg
[cratesio]: https://crates.io/crates/financial
[docsrs-img]: https://docs.rs/financial/badge.svg
[docsrs]: https://docs.rs/crate/financial

`Financial` is a [Rust](https://www.rust-lang.org/) crate that contains collection of finance calculations mimicking some of [Excel Financial Functions](https://support.microsoft.com/en-us/office/financial-functions-reference-5658d81e-6035-4f24-89c1-fbf124c2b1d8) interface.
you can find the crate [here](https://docs.rs/crate/financial)

## Usage

    use financial;

    let npv = financial::npv(0.1, &[-1000., 500., 500., 500.]);
    assert_eq!(npv, 221.29635953828267);

## What makes this crate different

It supports both periodic and scheduled computation for [IRR](https://en.wikipedia.org/wiki/Internal_rate_of_return) and [NPV](https://en.wikipedia.org/wiki/Net_present_value).

IRR and NPV functions are faster since powers are pre-computed iteratively instead of using power function multiple times. Take this with a grain of salt since no benches tests are offered at the moment.

## Supported Functions

- FV(Rate, Nper, Pmt, Pv, Pmt_is_due)
- PV(Rate, Nper, Pmt, Fv, Pmt_is_due)
- NPV(Rate, values)
- XNPV(Rate, values, dates)
- IRR(values)
- XIRR(values, dates)
- MIRR(values, finance_rate, reinvest_rate)

## NaiveDate Interface

- `financial::naive_date::xirr()` and `financial::naive_date::xnpv()` provide same functionalities as `financial::xirr()` and `financial::xnpv()`, except that the former supports `NaiveDate` as the input date type while the latter uses `DateTime<T>`.

## Future Work

- ~~Add bench tests~~
- Add More Functions (NPER, PMT, Rate, effect)

## Testing

- This crate has over 180 test case, most of them are compared to Excel outputs.
- XIRR is not compared against Excel, since Excel XIRR doesn't always converge to the correct answer and often produce the wrong answer of 0.000000002980.
Instead XIRR are tested by using the XIRR to produce a zero XNPV value.
- Note that the precision used for equality of floating points is 1e-7

## Contribution

- Using the crate and providing feedback or pointing out any issues.
- Adding more test cases is encouraged.
- Any contribution that serves the crate is welcome.
