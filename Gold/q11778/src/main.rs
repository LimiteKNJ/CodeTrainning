use std::io::{Write, BufWriter, self};

fn mmul_2x2 (matrix1 : Vec<Vec<u128>>, matrix2 : Vec<Vec<u128>>) -> Vec<Vec<u128>> {

    let mut result = vec![vec![0;2]; 2];
    result[0][0] = matrix1[0][0]*matrix2[0][0] % 1000000007 + matrix1[0][1]*matrix2[1][0] % 1000000007;
    result[0][1] = matrix1[0][0]*matrix2[0][1] % 1000000007 + matrix1[0][1]*matrix2[1][1] % 1000000007;
    result[1][0] = matrix1[0][0]*matrix2[1][0] % 1000000007 + matrix1[1][0]*matrix2[1][1] % 1000000007;
    result[1][1] = matrix1[0][1]*matrix2[1][0] % 1000000007 + matrix1[1][1]*matrix2[1][1] % 1000000007;

    return result;
}

fn fpow (matrix : Vec<Vec<u128>>, pow : u128) -> Vec<Vec<u128>> {

    if pow == 0 {
        return vec![vec![0,0],vec![0,0]];
    } else if pow == 1 {
        return vec![vec![1,1],vec![1,0]];
    } else {
        let tmp = fpow(matrix, pow/2);
        if pow % 2 == 0 {
            return mmul_2x2(tmp.clone(), tmp.clone());
            // n * n 
        } else {
            return mmul_2x2(mmul_2x2(tmp.clone(), tmp.clone()), vec![vec![1,1],vec![1,0]]);
            // n * n + 1
        }
    }
}

fn gcb(x : u128, y : u128) -> u128 {
    let mut _x = x;
    let mut _y = y;
    let mut _r: u128 = 0;
    while _y != 0 {
        _r = _x%_y;
        _x = _y;
        _y = _r;
    }
    return _x;
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let num1 = iter.next().unwrap().trim().parse::<u128>().unwrap();
    let num2 = iter.next().unwrap().trim().parse::<u128>().unwrap();
    let f1 = vec![vec![1,1],vec![1,0]];
    buf.clear();

    write!(out,"{}",fpow(f1.clone(), gcb(num1, num2))[0][1] % 1000000007).unwrap();
}