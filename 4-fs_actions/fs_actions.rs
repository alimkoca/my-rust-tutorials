use std::fs;

fn main() {
    println!("Reading file: test\n");

    let data = fs::read_to_string("test")
        .expect("error: can't read file");

    println!("Here is your poem, madam:\n{data}");
    println!("And say goodbye to your poem, because I'm overwriting it");

    fs::write("test", "No valid content\n")
        .expect("error: can't write file");
}
