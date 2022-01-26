use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no input");
    let num : usize = text.trim().parse().expect("Error");

    if num >= 90 {
        print!("A");
    } else if num >= 80 && num <= 89 {
        print!("B");
    } else if num >= 70 && num <= 79 {
        print!("C");
    } else if num >= 60 && num <= 69 {
        print!("D");
    } else {
        print!("F");
    }
}
