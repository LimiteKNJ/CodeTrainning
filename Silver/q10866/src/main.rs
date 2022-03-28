use std::collections::VecDeque;

fn main() {
    let mut deq : VecDeque<usize>  = VecDeque::new();

    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let excute_cnt : usize = bf.trim().parse().expect("Err");

    for _ in 0..excute_cnt {
        let mut bf2 = String::new();
        stdin.read_line(&mut bf2).expect("Err");
        let exe_cmd : Vec<String> = bf2.to_lowercase().split_whitespace()
                                        .map(|s|s.trim().parse().expect("Err"))
                                        .collect::<Vec<_>>();

        if !(exe_cmd.len() < 1) {
            if exe_cmd[0].eq("push_front") {
                if !(exe_cmd.len() < 2){
                    deq.push_front(exe_cmd[1].parse::<usize>().expect("Err"));
                } else { println!("Err") }

            } else if exe_cmd[0].eq("push_back") {
                if !(exe_cmd.len() < 2){
                    deq.push_back(exe_cmd[1].parse::<usize>().expect("Err"));
                } else { println!("Err") }

            } else if exe_cmd[0].eq("pop_front") {
                if deq.is_empty() { println!("-1") } else {
                    println!("{}", deq.pop_front().expect("Err"));
                }

            } else if exe_cmd[0].eq("pop_back") {
                if deq.is_empty() { println!("-1") } else {
                    println!("{}", deq.pop_back().expect("Err"));
                }

            } else if exe_cmd[0].eq("size") {
                println!("{}", deq.len());

            } else if exe_cmd[0].eq("empty") {
                if deq.is_empty() { println!("1") } else { println!("0") }

            } else if exe_cmd[0].eq("front") {
                if deq.is_empty() { println!("-1")} else {
                    println!("{}", deq[0]);
                }
            } else if exe_cmd[0].eq("back") {
                if deq.is_empty() { println!("-1")} else {
                    println!("{}", deq[deq.len() - 1]);
                }
            } else {
                println!("Err.");
            }
        } else {
            println!("Err.");
        }
    }
}