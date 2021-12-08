use std::io;
fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no input");
    let num : usize = text.trim().parse().expect("Input Error.");

    for i in 1..10{
        println!("{} * {} = {}", num , i, num*i);
    }
}
