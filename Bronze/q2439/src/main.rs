use std::io;
fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no Input.");
    let num : usize = text.trim().parse().expect("Input Error.");
    
    for i in (0..num).rev() {
        for j in 0..i {
            print!(" ");
        } for k in i..num {
            print!("*");
        }   println!();
    }
}
