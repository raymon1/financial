use chrono::{TimeZone, DateTime};

pub struct CheckedCashflowSchedule<'a, T> 
where T: TimeZone {
    pub values: &'a[f64],
    pub dates: &'a[DateTime<T>]
}

impl<'a, T> CheckedCashflowSchedule<'a, T> 
where T: TimeZone {
    pub fn new(values: &'a[f64], dates: &'a[DateTime<T>]) -> Result<CheckedCashflowSchedule<'a, T>, &'static str>
    where T: TimeZone {
        if values.len() != dates.len() {
            return Err("Values and dates length must match");
        }
        
        let d0 = dates.first().unwrap();
        if let Some(_) = dates.iter().position(|d| *d < *d0) {
            return Err("First date must be the earliest")
        };

        Ok(CheckedCashflowSchedule {
            values,
            dates,
        })
    }
}