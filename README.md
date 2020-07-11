# Financial

`Financial` is a [Rust](https://www.rust-lang.org/) crate that contains collection of finance calculations memicking some of [Excel Financial Functions](https://support.microsoft.com/en-us/office/financial-functions-reference-5658d81e-6035-4f24-89c1-fbf124c2b1d8) interface.

## What is different than other financial Rust cargos?

It supports both periodic and scheduled computation for [IRR](https://en.wikipedia.org/wiki/Internal_rate_of_return) and [NPV](https://en.wikipedia.org/wiki/Net_present_value).

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
