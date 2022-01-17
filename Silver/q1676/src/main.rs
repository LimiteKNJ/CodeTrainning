use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no Input.");
    let num : usize = text.trim().parse().expect("Input Error.");
    
    let mut count = 0;
    for _i in 1..num+1 {
        if (_i % 125) == 0 {
            count += 3;
        } else if (_i % 25) == 0 {
            count += 2;
        } else if (_i % 5) == 0 {
            count += 1;
        } else { continue; }
    }

    print!("{}",count);
}