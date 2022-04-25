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

    if pow <= 1 {
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
    let num = buf.trim().parse::<u128>().unwrap();
    let f1 = vec![vec![1,1],vec![1,0]];
    buf.clear();

    // Fn^2 = Fn * Fn = Fn * (Fn+1 - Fn-1)
    // Sum(0->n)Fi^2 = Sum(0->1)Fi * (Fi+1 - Fi-1) = = Sum(0->n)(FiFi+1 - FiFi-1)
    //              = Sum(0->n)FiFi+1 - Sum(0->n)FiFi-1 = FnFn+1
    write!(out,"{}",(fpow(f1.clone(), num)[0][1] * fpow(f1.clone(), num+1)[0][1]) % 1000000007).unwrap();
}