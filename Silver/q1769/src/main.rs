use std::io;
use io::Write;

fn sum(n : Vec<char>) -> usize {
    let mut sum = 0;

    for i in n {
        sum += i.to_digit(10).unwrap() as usize;
    }

    return sum;
}

fn main() {

    let mut buf = String::new();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut num = buf.trim_end().chars().collect::<Vec<_>>();

    let mut count = 0;
    let mut result = 0;

    if num.len() == 1 {
        result += num[0].to_digit(10).unwrap() as usize;
    }

    while num.len() > 1 {
        result = sum(num);
        num = result.to_string().trim().chars().collect::<Vec<_>>();
        count += 1;
    }

    if result % 3 == 0 {
        write!(out, "{}\nYES", count).unwrap();
    } else {
        write!(out, "{}\nNO", count).unwrap();
    }   
}