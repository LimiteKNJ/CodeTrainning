use std::io::{Write, BufWriter, self};

fn lcs(str1 : String, str2 : String, str3 : String) -> usize {
    let mut result = Vec::new(); // memozation
    let mut result_in = Vec::new();
    result_in.resize(str2.len()+1, vec![0; str3.len()+1]);
    result.resize(str1.len()+1, result_in);
    
    let str1_c = str1.trim().chars().collect::<Vec<char>>();
    let str2_c = str2.trim().chars().collect::<Vec<char>>();
    let str3_c = str3.trim().chars().collect::<Vec<char>>();
    for i in 1..=str1.len() {
        for j in 1..=str2.len(){
            for k in 1..=str3.len(){
                if str1_c[i-1] == str2_c[j-1] && str2_c[j-1] == str3_c[k-1] {
                    result[i][j][k] = result[i-1][j-1][k-1] + 1;
                } else {
                    result[i][j][k] = result[i-1][j][k].max(result[i][j-1][k].max(result[i][j][k-1]));
                }
            }
        }
    } return result[str1.len()][str2.len()][str3.len()];
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

    stdin.read_line(&mut buf).unwrap();
    let str3 = buf.trim().to_string();
    buf.clear();

    write!(out, "{}", lcs(str1, str2, str3)).unwrap();
}