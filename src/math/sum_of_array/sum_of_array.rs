pub fn sum_of_array(arr: Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in arr {
        sum += i;
    }
    return sum;
}
