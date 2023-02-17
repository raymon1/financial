use chrono::{DateTime, NaiveDate, Utc};

fn to_datetime(dates: &[NaiveDate]) -> Vec<DateTime<Utc>> {
    dates
        .iter()
        .map(|date| DateTime::<Utc>::from_utc(date.and_hms_opt(0, 0, 0).unwrap(), Utc))
        .collect::<Vec<DateTime<Utc>>>()
}

/// Returns the internal rate of return for a schedule of cash flows that is not necessarily periodic
///
/// This function is the same as `financial::xirr()` except that it uses `NaiveDate` as the inupt type.
///
/// # Example
/// ```
/// use chrono::{NaiveDate};
/// let cf = [-379., 100., 100., 100., 100., 100.];
/// let dates = [
///     NaiveDate::from_ymd(2016, 7, 8),
///     NaiveDate::from_ymd(2017, 7, 8),
///     NaiveDate::from_ymd(2018, 7, 8),
///     NaiveDate::from_ymd(2019, 7, 8),
///     NaiveDate::from_ymd(2020, 7, 8),
///     NaiveDate::from_ymd(2021, 7, 8),
/// ];
/// assert!((financial::naive_date::xirr(&cf, &dates, None).unwrap() - 0.10004608364).abs() < 1e-7);
/// ```
pub fn xirr(values: &[f64], dates: &[NaiveDate], guess: Option<f64>) -> Result<f64, &'static str> {
    crate::scheduled_cashflow::xirr::xirr(values, &to_datetime(dates), guess)
}

/// Returns the net present value for a schedule of cash flows that is not necessarily periodic.
///
/// This function is the same as `financial::xirr()` except that it uses `NaiveDate` as the inupt type.
///
/// # Example
/// ```
/// use chrono::{NaiveDate, DateTime, Utc, Duration};
/// let cf = [-500., 100., 100., 100., 100., 100.];
/// let dates = [
///     NaiveDate::from_ymd(2016, 7, 8),
///     NaiveDate::from_ymd(2017, 7, 8),
///     NaiveDate::from_ymd(2018, 7, 8),
///     NaiveDate::from_ymd(2019, 7, 8),
///     NaiveDate::from_ymd(2020, 7, 8),
///     NaiveDate::from_ymd(2021, 7, 8),
/// ];
/// assert_eq!(financial::naive_date::xnpv(0.1, &cf, &dates).unwrap(), -120.9553674519204);
/// ```
pub fn xnpv(rate: f64, values: &[f64], dates: &[NaiveDate]) -> Result<f64, &'static str> {
    crate::scheduled_cashflow::xnpv::xnpv(rate, values, &to_datetime(dates))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, NaiveDate};
    #[test]
    fn xnpv_with_zero_rate() {
        let cf: [f64; 10000] = [100.; 10000];
        let somedate: NaiveDate = NaiveDate::from_ymd(2021, 1, 1);
        let dates0: [NaiveDate; 10000] = [somedate; 10000];
        let mut dates: [NaiveDate; 10000] = [somedate; 10000];

        let mut i = 0;
        for d in dates0.iter() {
            dates[i] = d
                .checked_add_signed(Duration::weeks(52 * (i as i64)))
                .unwrap();
            i = i + 1;
        }

        assert_eq!(xnpv(0., &cf, &dates).unwrap(), cf.iter().sum());
    }

    #[test]
    fn xnpv_test() {
        let cf = [-500., 100., 100., 100., 100., 100.];
        let dates = [
            NaiveDate::from_ymd(2016, 7, 8),
            NaiveDate::from_ymd(2017, 7, 8),
            NaiveDate::from_ymd(2018, 7, 8),
            NaiveDate::from_ymd(2019, 7, 8),
            NaiveDate::from_ymd(2020, 7, 8),
            NaiveDate::from_ymd(2021, 7, 8),
        ];
        assert_eq!(xnpv(0.1, &cf, &dates).unwrap(), -120.9553674519204);
    }
}
