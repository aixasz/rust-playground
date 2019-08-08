fn main() {
    for x in 0..101 {
        let result = if x % 15 == 0 {
            "FizzBuzz"
        } else if x % 3 == 0 {
           "Fizz"
        } else if x % 5 == 0 {
           "Buzz"
        } else {
            ""
        };  

        println!("{} : {}", result, x);   
    }
}
