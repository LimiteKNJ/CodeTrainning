fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let mut time : Vec<u8> = bf.split_whitespace()
                            .map(|s|s.trim().parse().expect("Err"))
                            .collect::<Vec<_>>();
    
    if !(time.len() < 2) {
        if time[1] < 45 {
            time[1] += 15;
            if time[0] != 0 { time[0] -= 1; } else { time[0] = 23 };
        } else {
            time[1] -= 45;
        } print! ("{} {}", time[0], time[1]);
    } else { panic!("Err"); }
}