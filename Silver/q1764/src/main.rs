use std::io;
use io::Write;
use std::collections::HashSet;

fn main() {
    
    let stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let mut out = io::BufWriter::new(stdout.lock());
    
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let not_listen_num = iter.next().unwrap().parse::<usize>().unwrap();
    let not_look_num = iter.next().unwrap().parse::<usize>().unwrap();
    buf.clear();

    let mut not_listen_people : HashSet<String> = HashSet::new();
    let mut not_look_people : HashSet<String> = HashSet::new();

    for _ in 0..not_listen_num {
        stdin.read_line(&mut buf).unwrap();
        not_listen_people.insert(buf.trim().to_string());
        buf.clear();
    }

    for _ in 0..not_look_num {
        stdin.read_line(&mut buf).unwrap();
        not_look_people.insert(buf.trim().to_string());
        buf.clear();
    }
    
    let mut not_listen_look_people : Vec<&String> = not_listen_people.intersection(&not_look_people).collect::<Vec<_>>();
    not_listen_look_people.sort();

    write!(out, "{}\n", not_listen_look_people.len()).unwrap();
    for n in not_listen_look_people {
        write!(out, "{}\n", n).unwrap();
    }
}