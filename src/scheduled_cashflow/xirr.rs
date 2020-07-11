use chrono::{DateTime, TimeZone};
use crate::common::find_root::find_root;
use crate::scheduled_cashflow::xnpv::calculate_xnpv;
use crate::scheduled_cashflow::CheckedCashflowSchedule;

///
/// 
/// # Example
/// ``` 
/// use chrono::{NaiveDate, DateTime, Utc};
/// let cf = [-379., 100., 100., 100., 100., 100.];
/// let dates = [
///     DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 0, 0), Utc),
///     DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2017, 7, 8).and_hms(0, 0, 0), Utc),
///     DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2018, 7, 8).and_hms(0, 0, 0), Utc),
///     DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 7, 8).and_hms(0, 0, 0), Utc),
///     DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2020, 7, 8).and_hms(0, 0, 0), Utc),
///     DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 7, 8).and_hms(0, 0, 0), Utc), 
/// ];
/// assert_eq!(financial::xirr(&cf, &dates, &None).unwrap(), 0.10004608364551085);  
/// ``` 
pub fn xirr<T: TimeZone>(values: &[f64], dates: &[DateTime<T>], guess: &Option<f64>) -> Result<f64, &'static str> {
    let cf = CheckedCashflowSchedule::new(values, dates);

    match cf {
        Err(m) => Err(m),
        Ok(cf) => {
            let f_xnpv = |x: f64| calculate_xnpv(&x, &cf);
            match find_root(&guess, f_xnpv) {
                Some(ans) => Ok(ans),
                None => Err("could't find irr for the values provided")
            }
        }
    }    
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, DateTime, Utc};

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
        assert_eq!(xirr(&cf, &dates, &None).unwrap(), 0.);        
    }

    #[test]
    fn xnpv_test2() {
        let cf = [-379., 100., 100., 100., 100., 100.];
        let dates = [
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 0, 0), Utc),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2017, 7, 8).and_hms(0, 0, 0), Utc),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2018, 7, 8).and_hms(0, 0, 0), Utc),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 7, 8).and_hms(0, 0, 0), Utc),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2020, 7, 8).and_hms(0, 0, 0), Utc),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 7, 8).and_hms(0, 0, 0), Utc), 
        ];
        assert_eq!(xirr(&cf, &dates, &None).unwrap(), 0.10004608364551086);        
    }
}