#[cfg(test)]
mod tests {

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
            assert!((ans - res).abs() < PRECISION, "case {}, answer is {}, result is {}", case_index, ans, res);
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

            if !(ans.is_nan() ) {
                assert!((ans - res).abs() < PRECISION, "cases {}, answer is {}, result is {}", case_index, ans, res);
            }
        });
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