use std::io::stdin;
use std::io::stdout;
use std::io::Write;

fn fact(num: u32) -> u32 {
    if num == 0 {
        return 1;
    }
    return num * fact(num-1);
}

fn main() {
    let mut input = String::new();

    print!("Which num do you want to take factorial of?: ");
    let _ = stdout().flush();

    stdin().read_line(&mut input)
        .expect("Error");

    // Note: Don't forget trim for \r and \n, 
    // these are can't be unwrapped.
    let num: u32 = input.trim().parse().unwrap();
    println!("{}", fact(num));
}
