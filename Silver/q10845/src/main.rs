use std::collections::VecDeque;

fn main() {
    let mut queue : VecDeque<usize>  = VecDeque::new();

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
            if exe_cmd[0].eq("push") {
                if !(exe_cmd.len() < 2){
                    queue.push_back(exe_cmd[1].parse::<usize>().expect("Err"));
                } else { println!("Err") }

            } else if exe_cmd[0].eq("pop") {
                if queue.is_empty() { println!("-1") } else {
                    println!("{}", queue.pop_front().expect("Err"));
                }

            } else if exe_cmd[0].eq("size") {
                println!("{}", queue.len());

            } else if exe_cmd[0].eq("empty") {
                if queue.is_empty() { println!("1") } else { println!("0") }

            } else if exe_cmd[0].eq("front") {
                if queue.is_empty() { println!("-1")} else {
                    println!("{}", queue[0]);
                }
            } else if exe_cmd[0].eq("back") {
                if queue.is_empty() { println!("-1")} else {
                    println!("{}", queue[queue.len() - 1]);
                }
            } else {
                println!("Err.");
            }
        } else {
            println!("Err.");
        }
    }
}