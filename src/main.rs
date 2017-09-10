fn main() {
    let max = 100;
    for num in 1..max + 1 {
        if num % 15 == 0 {
            print!(" fizzbuzz");
        }
        else if num % 3 == 0 {
            print!(" fizz");
        }
        else if num % 5 == 0 {
            print!(" buzz");
        }
        else {
            print!(" {}", num.to_string());
        }

    }
}
