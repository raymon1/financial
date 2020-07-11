use chrono::{DateTime, offset::TimeZone};
use crate::common::utils;

/// Returns the net present value for a schedule of cash flows that is not necessarily periodic.
pub fn xnpv<T: TimeZone>(rate: &f64, values: &[f64], dates: &[DateTime<T>]) -> Result<f64, &'static str> {
    if values.len() != dates.len() {
        return Err("Values and dates length must match");
    }

    if values.len() == 0 {
        // should i?
        return Ok(0.);
    }

    let d0 = dates.first().unwrap();
    if let Some(_) = dates.iter().position(|d| *d < *d0) {
        return Err("First date must be the earliest")
    };

    if *rate == 0. { 
        return Ok(values.iter().sum());
    }

    Ok(
        values.iter()
        .zip(dates.iter())
        .map(|(v, d)| v / f64::powf(1. + rate, utils::days_to(d0.clone(), d.clone()) / 365.))
        .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, DateTime, Utc, Duration};

    #[test]
    fn xnpv_with_zero_rate() {
        let cf: [f64; 10000] = [100.; 10000];
        let dates0: [DateTime<Utc>; 10000] = [Utc::now(); 10000];
        let mut dates: [DateTime<Utc>; 10000] = [Utc::now(); 10000];

        let mut i = 0;
        for d in dates0.iter() {
            dates[i] = d.checked_add_signed(Duration::weeks(52 * (i as i64))).unwrap();
            i = i + 1;
        }

        assert_eq!(xnpv(&0., &cf, &dates).unwrap(), cf.iter().sum());        
    }

    #[test]
    fn xnpv_test() {
        let cf = [-500., 100., 100., 100., 100., 100.];
        let dates = [
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 0, 0), Utc),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2017, 7, 8).and_hms(0, 0, 0), Utc),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2018, 7, 8).and_hms(0, 0, 0), Utc),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 7, 8).and_hms(0, 0, 0), Utc),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2020, 7, 8).and_hms(0, 0, 0), Utc),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 7, 8).and_hms(0, 0, 0), Utc), 
        ];
        assert_eq!(xnpv(&0.1, &cf, &dates).unwrap(), -120.9553674519204);        
    }
}