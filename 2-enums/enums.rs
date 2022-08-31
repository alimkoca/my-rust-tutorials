use std::io::stdout;
use std::io::stdin;
use std::io::Write;

fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String)
    }

    let mut home = String::new();
    let mut loopback = String::new();

    print!("Please enter home IP Address: ");
    let _ = stdout().flush();

    stdin().read_line(&mut home)
        .expect("Error!");

    print!("Please enter loopback IP Address: ");
    let _ = stdout().flush();

    stdin().read_line(&mut loopback)
        .expect("Error!");

    let home = IpAddr::V4(String::from(home));
    let loopback = IpAddr::V6(String::from(loopback));

    println!("{home:?}, {loopback:?}");
}
