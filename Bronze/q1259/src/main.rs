fn main() {
    while true {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let num = buf.trim().chars().collect::<String>();

        if num == '0'.to_string() {
            break;
        } else {
            let rev_num = num.chars().rev().collect::<String>();
            if num.eq(&rev_num) {
                println!("yes");
            } else {
                println!("no");
            }
        }
    }
}
