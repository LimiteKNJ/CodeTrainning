use std::io::{Write, BufWriter, self};

fn bfs(start : u128, end : u128) -> isize {

    let mut arrive = true;
    let mut cnt = 1;
    let mut tmp = end;

    while arrive {
        if tmp < start {
           arrive = false; break;

        } else if tmp == start {
            arrive = true; break;
        
        } else {
            if tmp % 10 == 1 {
                tmp = tmp / 10;
                cnt += 1;
    
            } else if tmp % 2 == 0 {
                tmp = tmp / 2;
                cnt += 1;
                
            } else {
                arrive = false; break;
            } 
        }
    }

    if arrive {
        return cnt;
    } else {
        return -1;
    } 
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let start = iter.next().unwrap().parse::<u128>().unwrap();
    let end = iter.next().unwrap().parse::<u128>().unwrap();
    buf.clear();

    write!(out,"{}", bfs(start, end)).unwrap();
}
