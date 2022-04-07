// 푸는 원리는 알겠는데, 더블벡터가 맞는가?

use std::io;
use std::cmp;
use io::Write;

fn cmp_wb (board : &Vec<Vec<char>>, x : usize, y : usize) -> usize {

    let mut cnt = 0;
    let wb = vec![
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W']
    ];

    for i in 0..8 {
        for j in 0..8 {
            if board[x+i][y+j] != wb[i][j] {
                cnt += 1;
            }
        }
    }
    return cnt;
}

fn cmp_bb (board : &Vec<Vec<char>>, x : usize, y : usize) -> usize {

    let mut cnt = 0;
    let bb = vec![
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'], 
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'], 
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'], 
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B']
    ];

    for i in 0..8 {
        for j in 0..8 {
            if board[x+i][y+j] != bb[i][j] {
                cnt += 1;
            }
        }
    }

    return cnt;
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let m = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let n = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    let mut chess_board : Vec<Vec<char>> = Vec::new();
    for _ in 0..m {
        stdin.read_line(&mut buf).unwrap();
        let chess_input_line = buf.trim().chars().collect::<Vec<_>>();
        chess_board.push(chess_input_line);
        buf.clear();
    }

    let mut num = 0;
    let mut min = m*n+1;
    for i in 0..m-7 {
        for j in 0..n-7 {
            num = cmp::min(cmp_wb(&chess_board, i, j), cmp_bb(&chess_board, i, j));
            if num < min { min = num; }
        }
    } write!(out, "{}", min).unwrap();
}