use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Error");
    let size : usize = text.trim().parse().expect("parse Err.");

    for i in 0..size{
        let mut temp = [0 ; 1];
        
   }
}
