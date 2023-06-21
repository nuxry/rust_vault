pub fn sum_of_series(series: Vec<i32>, n: u32) -> i32{
    let a = series[0];
    let r = series[1] / series[0];

    if r != 1 {
        return a * (1 - r.pow(n)) / (1 - r);
    } else {
        panic!("Invalid value!");
    }
}