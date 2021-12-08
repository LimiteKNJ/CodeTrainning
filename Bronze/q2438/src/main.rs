use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("no Input");
    let num : usize = s.trim().parse().expect("Input Error.");
    let mut text = String::new();

    for i in 0..num{
        text.push('*');
        println!("{}", text);
    }
}

/*
use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("no Input");
    let num : usize = s.trim().parse().expect("Input Error.");

    for i in 0..num{
        for j in 0..i+1{
            print!("*");
        } println!();
    }
}
*/