use std::ops::Range;

fn main() {
    println!("The fizzbuzz output:");
    let mut output: String = "".into();
    let range = Range { start: 1, end: 100 };
    for num in range {
        if num % 3 == 0 {
            output = output + "fizz";
        }
        if num % 5 == 0 {
            output = output + "buzz";
        }
        if output.len() == 0 {
            output = num.to_string();
        }
    }
    print!("{}", output);
}