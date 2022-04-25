use std::io::{Write, BufWriter, self};

fn mmul_2x2 (matrix1 : Vec<Vec<u128>>, matrix2 : Vec<Vec<u128>>) -> Vec<Vec<u128>> {

    let mut result = vec![vec![0;2]; 2];
    result[0][0] = matrix1[0][0]*matrix2[0][0] % 1000000000 + matrix1[0][1]*matrix2[1][0] % 1000000000;
    result[0][1] = matrix1[0][0]*matrix2[0][1] % 1000000000 + matrix1[0][1]*matrix2[1][1] % 1000000000;
    result[1][0] = matrix1[0][0]*matrix2[1][0] % 1000000000 + matrix1[1][0]*matrix2[1][1] % 1000000000;
    result[1][1] = matrix1[0][1]*matrix2[1][0] % 1000000000 + matrix1[1][1]*matrix2[1][1] % 1000000000;

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

    // Sum(m->n)Fi = Sum(0->n)Fi - Sum(0->m)Fi
    //             = (Fn+2 - 1) - (Fm+1 - 1) = Fn+2 - Fm+1
    write!(out,"{}",(fpow(f1.clone(), num2+2)[0][1] - fpow(f1.clone(), num1+1)[0][1] + 1000000000) % 1000000000).unwrap();
}