use std::io::{Write, BufWriter, self};
fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut board = buf.trim();
    let mut result = String::new();
    let mut possible = true;

    while board.contains("X"){
        if let Some(dot) = board.find("."){
            let mut temp = &board[..dot];
            board = &board[dot+1..];

            while temp.contains("X"){
                if let Some(aaaa_idx) = temp.find("XXXX"){
                    result.push_str("AAAA");
                    temp = &temp[aaaa_idx+4..];
                } else if let Some(bb_idx) = temp.find("XX"){
                    result.push_str("BB");
                    temp = &temp[bb_idx+2..];
                } else {
                    possible = false; break;
                }
            } result.push_str(".");

        } else {
            if let Some(aaaa_idx) = board.find("XXXX"){
                result.push_str("AAAA");
                board = &board[aaaa_idx+4..];
            } else if let Some(bb_idx) = board.find("XX"){
                result.push_str("BB");
                board = &board[bb_idx+2..];
            } else {
                possible = false; break;
            }
        }
    }

    if board.contains("."){
        result.push_str(board);
    }

    if !possible {
        write!(out, "-1").unwrap();
    } else {
        write!(out, "{}", result).unwrap();
    }
}
