use core::num;
use std::io::{Write, BufWriter, self};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let size = iter.next().unwrap().parse::<usize>().unwrap();
    let case = iter.next().unwrap().parse::<usize>().unwrap();
    buf.clear();

    let mut mat_vec : Vec<Vec<isize>> = Vec::new();
    for _ in 1..=size {
        stdin.read_line(&mut buf).unwrap();
        let line_vec = buf.split_whitespace()
        .map(|s|s.trim().parse::<isize>().unwrap())
        .collect::<Vec<_>>(); mat_vec.push(line_vec);
        buf.clear();
    }

    for i in 0..size {
        for j in 0..size {
            if i == 0 && j == 0 {
                continue;
            } else if i == 0 && j != 0 {
                mat_vec[0][j] += mat_vec[0][j-1];
            } else if i != 0 && j == 0 {
                mat_vec[i][0] += mat_vec[i-1][0];
            } else {
                let sum = mat_vec[i][j] + mat_vec[i-1][j] + mat_vec[i][j-1] - mat_vec[i-1][j-1];
                mat_vec[i][j] = sum;
            }
        }
    }
    
    for _ in 0..case {
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let x1 = iter.next().unwrap().parse::<usize>().unwrap() -1;
        let y1 = iter.next().unwrap().parse::<usize>().unwrap() -1;
        let x2 = iter.next().unwrap().parse::<usize>().unwrap() -1;
        let y2 = iter.next().unwrap().parse::<usize>().unwrap() -1;

        if x1 == 0 && y1 == 0 {
            write!(out, "{}\n", mat_vec[x2][y2]).unwrap();
        } else if x1 != 0 && y1 == 0 {
            write!(out, "{}\n", mat_vec[x2][y2] - mat_vec[x1-1][y2]).unwrap();
        } else if x1 == 0 && y1 != 0 {
            write!(out, "{}\n", mat_vec[x2][y2] - mat_vec[x2][y1-1]).unwrap();
        } else {
            write!(out, "{}\n", mat_vec[x2][y2] - (mat_vec[x1-1][y2] + mat_vec[x2][y1-1] - mat_vec[x1-1][y1-1])).unwrap();
        } buf.clear();
    }
}