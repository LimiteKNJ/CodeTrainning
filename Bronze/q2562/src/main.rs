use std::io;
fn main() {
    // Input One Line
    let mut num = Vec::new();
    let mut max: usize = 0;
    let mut i = 0;

    loop {
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("no input");
        let temp : usize = text.trim().parse().expect("Input Error.");
        num.push(temp); i += 1; if i == 9 {break;}
    }

    let mut index = 0;
    for j in 0..9 {
        if num[j] > max {
            max = num[j]; index = j + 1;
        }
    }
    println!("{}", max);
    println!("{}", index);
}
