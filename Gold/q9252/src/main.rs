use std::io::{Write, BufWriter, self};

fn lcs(str1 : String, str2 : String) -> (usize, String) {
    let mut result = Vec::new(); // memozation
    let mut str_rev = Vec::new();
    result.resize(str1.len()+1, vec![0; str2.len()+1]);
    
    let str1_c = str1.trim().chars().collect::<Vec<char>>();
    let str2_c = str2.trim().chars().collect::<Vec<char>>();
    for i in 1..=str1.len() {
        for j in 1..=str2.len(){
            if str1_c[i-1] == str2_c[j-1] {
                result[i][j] = result[i-1][j-1] + 1;
            } else {
                result[i][j] = result[i-1][j].max(result[i][j-1]);
            }
        }
    }
    
    let mut col = str1.len();
    let mut row = str2.len();
    while result[col][row] != 0 {
        if result[col][row] == result[col][row-1]{
            row -= 1;
        } else if result[col][row] == result[col-1][row]{
            col -= 1;
        } else {
            str_rev.push(str1_c[col-1]);
            col -= 1; row -= 1;
        }
    }
    
    let mut str = String::new();
    for i in str_rev.into_iter().rev() {
        str.push(i);
    } return (result[str1.len()][str2.len()], str);
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let str1 = buf.trim().to_string();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let str2 = buf.trim().to_string();
    buf.clear();

    let result = lcs(str1, str2);
    write!(out, "{}\n{}", result.0, result.1).unwrap();
}