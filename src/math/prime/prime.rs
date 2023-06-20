pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as u32) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}