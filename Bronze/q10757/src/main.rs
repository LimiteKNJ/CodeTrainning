use std::io::{Write, BufWriter, self};

fn bigint_sum(mut a : Vec<u32>, mut b : Vec<u32>) -> Vec<u32> {

    if a.len() > b.len() {
        for _ in 0..a.len()-b.len() {
            b.push(0);
        }
    } else if a.len() < b.len() {
        for _ in 0..b.len()-a.len() {
            a.push(0);
        }
    }

    let mut result = vec![0; a.len()];
    let mut carry= 0;

    for i in 0..a.len() {
        result[i] = (a[i] + b[i] + carry) % 10;
        if (a[i] + b[i] + carry) / 10 == 1 {
            carry = (((a[i] + b[i] + carry) / 10) as f64).floor() as u32;
            if i == a.len()-1 {
                if carry != 0 { result.push(carry); }
            }
        } else { carry = 0; }
    }
    
    while result[result.len()-1] == 0 {
        result.pop().unwrap();
    }
    
    return result;
}

fn main() {  
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let num1 = iter.next().unwrap().chars().map(|s|s.to_digit(10).unwrap()).rev()
                                                .collect::<Vec<_>>();
    let num2 = iter.next().unwrap().chars().map(|s|s.to_digit(10).unwrap()).rev()
                                                .collect::<Vec<_>>();
    buf.clear();

    let result = bigint_sum(num1, num2);
    for i in result.into_iter().rev() {
        write!(out, "{}", i).unwrap();
    }
}