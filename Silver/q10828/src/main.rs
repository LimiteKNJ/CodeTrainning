fn main() {
    let mut stack = Vec::new();

    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let excute_cnt : u8 = bf.trim().parse().expect("Err");

    for i in 0..excute_cnt {
        let mut bf2 = String::new();
        stdin.read_line(&mut bf2).expect("Err");
        let exe_cmd : Vec<String> = bf2.to_lowercase().split_whitespace()
                                        .map(|s|s.trim().parse().expect("Err"))
                                        .collect::<Vec<_>>();

        if !(exe_cmd.len() < 1) {
            if exe_cmd[0].eq("push") {
                if !(exe_cmd.len() < 2){
                    stack.push(exe_cmd[1].to_string());
                } else { panic!("Index Out of Bound"); }

            } if exe_cmd[0].eq("pop") {
                if stack.is_empty() { print!("-1") } else {
                    print!("{}", stack.pop().expect("-1"));
                }

            } if exe_cmd[0].eq("size") {
                print!("{}", stack.len());

            } if exe_cmd[0].eq("empty") {
                if stack.is_empty() { print!("0") } else { print!("1")}

            } if exe_cmd[0].eq("top") {
                if stack.is_empty() { print!("-1")} else {
                    print!("{}", stack[stack.len() - 1]);
                }
            }
        }
    }
}
