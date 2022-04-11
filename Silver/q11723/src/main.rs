use std::io;
use io::Write;

fn main() {
    let mut set : Vec<bool>  = vec![false; 21];

    let mut bf = String::new();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    stdin.read_line(&mut bf).expect("Err");
    let excute_cnt : usize = bf.trim().parse().expect("Err");
    bf.clear();

    for _ in 0..excute_cnt {
        stdin.read_line(&mut bf).expect("Err");
        let exe_cmd : Vec<String> = bf.to_lowercase().split_whitespace()
                                        .map(|s|s.trim().parse().expect("Err"))
                                        .collect::<Vec<_>>();
        bf.clear();

        if !(exe_cmd.len() < 1) {
            if exe_cmd[0].eq("add") {
                if !(exe_cmd.len() < 2){
                    set[exe_cmd[1].parse::<usize>().expect("Err")] = true;
                } else { write!(out, "Err\n").unwrap(); }

            } else if exe_cmd[0].eq("remove") {
                if !(exe_cmd.len() < 2){
                    set[exe_cmd[1].parse::<usize>().expect("Err")] = false;
                } else { write!(out, "Err\n").unwrap(); }

            } else if exe_cmd[0].eq("check") {
                if !(exe_cmd.len() < 2){
                    if set[exe_cmd[1].parse::<usize>().expect("Err")] {
                        write!(out,"1\n").unwrap();
                    } else { write!(out,"0\n").unwrap(); }
                } else { write!(out, "Err\n").unwrap(); }

            } else if exe_cmd[0].eq("toggle") {
                if !(exe_cmd.len() < 2){
                    if set[exe_cmd[1].parse::<usize>().expect("Err")] {
                        set[exe_cmd[1].parse::<usize>().expect("Err")] = false;
                    } else {
                        set[exe_cmd[1].parse::<usize>().expect("Err")] = true;
                    }
                } else { write!(out, "Err\n").unwrap(); }

            } else if exe_cmd[0].eq("all") {
                set.fill(true);

            } else if exe_cmd[0].eq("empty") {
                set.fill(false);

            }  else {
                write!(out, "Err\n").unwrap(); 
            }
        } else {
            write!(out, "Err\n").unwrap(); 
        }
    }
}