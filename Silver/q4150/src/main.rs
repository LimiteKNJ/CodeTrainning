use std::io::{Write, BufWriter, self};

/* Same Code for Q10826 */

fn fib(n : u128) -> Vec<usize> {

    let mut result = Vec::new();

    if n == 0 {
        return vec![0];
    } else if n == 1 {
        return vec![1];
    } else {
        let mut num1 = vec![0];
        let mut num2 = vec![1];
        result = bigint_sum(num1.clone(), num2.clone());
        for _ in 2..=n {
            num2.clone_from(&num1);
            num1.clone_from(&result);
            result = bigint_sum(num1.clone(), num2.clone());
        }
    }

    return result;
}

fn bigint_sum(mut a : Vec<usize>, mut b : Vec<usize>) -> Vec<usize> {

    let mut result = vec![0; a.len()];
    let mut carry= 0;

    if a.len() > b.len() {
        for _ in 0..a.len()-b.len() {
            b.push(0);
        }
    } else if a.len() < b.len() {
        for _ in 0..b.len()-a.len() {
            a.push(0);
        }
    }

    for i in 0..a.len() {
        result[i] = (a[i] + b[i] + carry) % 10;
        if (a[i] + b[i] + carry) / 10 == 1 {
            carry = (((a[i] + b[i] + carry) / 10) as f64).floor() as usize;
            if i == a.len()-1 {
                if carry != 0 { result.push(carry); }
            }
        } else { carry = 0; }
    } return result;
}

fn main() {
    
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<u128>().unwrap();

    let result = fib(num);
    for i in result.into_iter().rev() {
        write!(out, "{}", i).unwrap();
    }
}