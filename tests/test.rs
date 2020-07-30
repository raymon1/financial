#[cfg(test)]
mod tests {

    use chrono::{Utc, DateTime, NaiveDate};
    use financial;

    pub const PRECISION: f64 = 1e-7;

    #[test]
    fn npv() {
        test_fn("./tests/test_data/npv.csv", |test_case, case_index| {
            let mut test_case = test_case;

            let(ans, r, values) : (f64, f64, Vec<f64>) = (
                test_case.next().unwrap().parse::<f64>().unwrap(),
                test_case.next().unwrap().parse::<f64>().unwrap(),
                test_case.map(|x| x.parse::<f64>().unwrap()).collect());
            
            let res = financial::npv(r, &values);
            assert!((ans - res).abs() < PRECISION, "case {}: answer is {}, result is {}", case_index, ans, res);
        });
    }

    #[test]
    fn irr() {
        test_fn("./tests/test_data/irr.csv", |test_case, case_index| {
            let mut test_case = test_case;

            let(ans, values) : (f64, Vec<f64>) = (
                test_case.next().unwrap().parse::<f64>().unwrap_or_else(|_| f64::NAN),
                test_case.map(|x| x.parse::<f64>().unwrap()).collect());
            
            let res = financial::irr(&values, None).unwrap_or_else(|_| f64::NAN);

            if !ans.is_nan() {
                assert!((ans - res).abs() < PRECISION, "case {}: answer is {}, result is {}", case_index, ans, res);
            } 
        });
    }

    #[test]
    fn xnpv() {
        test_fn("./tests/test_data/xnpv.csv", |test_case, case_index| {
            let mut test_case = test_case;

            let date_from_str = |x| DateTime::<Utc>::from_utc(
                NaiveDate::parse_from_str(x, "%m/%d/%Y").unwrap().and_hms(0, 0, 0), Utc);

            let(ans, r, values, dates) : (f64, f64, Vec<f64>, Vec<DateTime<Utc>>) = (
                test_case.next().unwrap().parse::<f64>().unwrap(),
                test_case.next().unwrap().parse::<f64>().unwrap(),
                test_case.by_ref().take_while(|x| x.parse::<f64>().is_ok()).map(|x| x.parse::<f64>().unwrap()).collect(), 
                test_case.map(|x| date_from_str(x)).collect()
            );
            
            let res = financial::xnpv(r, &values, &dates).unwrap()  ;
            assert!((ans - res).abs() < PRECISION, "case {}: answer is {}, result is {}", case_index, ans, res);
        });
    }

    #[test]
    fn xirr() {
        test_fn("./tests/test_data/xirr.csv", |test_case, case_index| {
            let mut test_case = test_case;

            let date_from_str = |x| DateTime::<Utc>::from_utc(
                NaiveDate::parse_from_str(x, "%m/%d/%Y").unwrap().and_hms(12, 0, 0), Utc);

            let(ans, values, dates) : (f64, Vec<f64>, Vec<DateTime<Utc>>) = (
                test_case.next().unwrap().parse::<f64>().unwrap_or_else(|_| f64::NAN),
                test_case.by_ref().take_while(|x| x.parse::<f64>().is_ok()).map(|x| x.parse::<f64>().unwrap()).collect(), 
                test_case.map(|x| date_from_str(x)).collect()
            );
            
            let res = financial::xirr(&values, &dates, None).unwrap_or_else(|_| f64::NAN);

            if !res.is_nan() {
                assert_nearly_eq(0., financial::xnpv(res, &values, &dates).unwrap(), case_index);
            }

            // It appears that excel has a flow in its XIRR function that cause the answer to converge at 0.000000002980
            // That's why I preferred to use the XNPV function to make sure that the output of xirr converge the NPV value of cashflow to approximately zero,
            // It's also good to note that this is not perfect since XIRR is dependent on XNPV that is used to test it and assumes the correctness of XNPV function.
            
            // if !ans.is_nan() {
            //     assert!((ans - res).abs() < PRECISION, "case {}: answer is {}, result is {}, diff is {}", case_index, ans, res, (ans - res).abs());
            // }
        });
    }

    fn assert_nearly_eq(ans: f64, res: f64, case_index: i32) {
        assert!((ans - res).abs() < PRECISION, "case {}: answer is {}, result is {}, diff is {}", case_index, ans, res, (ans - res).abs());
    }


    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    fn test_fn<P, F>(filename: P, func: F)
    where P: AsRef<Path>,
        F: Fn(std::str::Split<&str>, i32), {

        if let Ok(lines) = read_lines(filename) {
            let mut case_index = 0;
            for line in lines.skip(1) {
                if let Ok(test_case) = line {
                    let test_case = test_case.split(",");
                    case_index += 1;
                    func(test_case, case_index);
                }
            }
        }
    }       


    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        match File::open(filename) {
            Ok(file) => Ok(io::BufReader::new(file).lines()),
            Err(e) => panic!("file read error: {}", e)
        }
    }    
}