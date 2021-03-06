fn main() {
    let mut stack : Vec<usize>  = Vec::new();

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
                    stack.push(exe_cmd[1].parse::<usize>().expect("Err"));
                } else { println!("Err") }

            } else if exe_cmd[0].eq("pop") {
                if stack.is_empty() { println!("-1") } else {
                    println!("{}", stack.pop().expect("Err"));
                }

            } else if exe_cmd[0].eq("size") {
                println!("{}", stack.len());

            } else if exe_cmd[0].eq("empty") {
                if stack.is_empty() { println!("1") } else { println!("0") }

            } else if exe_cmd[0].eq("top") {
                if stack.is_empty() { println!("-1")} else {
                    println!("{}", stack[stack.len() - 1]);
                }
            } else {
                println!("Err.");
            }
        } else {
            println!("Err.");
        }
    }
}