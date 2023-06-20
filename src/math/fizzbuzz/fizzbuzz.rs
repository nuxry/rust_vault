pub fn fizzbuzz(n: i32) {
    if n % 3 == 0 && n % 5 == 0 {
        println!("{n} : FizzBuzz");
    } else if n % 3 == 0 {
        println!("{n} : Fizz");
    } else if n % 5 == 0 {
        println!("{n} : Buzz");
    }
}
