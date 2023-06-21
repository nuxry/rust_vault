pub fn sum_of_series(series: Vec<i32>, n: i32) -> i32{
    let a: i32 = series[0];
    let d: i32 = series[1] - series[0];

    let sum = n / 2 * ((2 * a) + ((n - 1) * d));

    return sum;
}
