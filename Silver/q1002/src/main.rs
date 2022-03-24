fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let case : usize = bf.trim().parse().expect("Err");
    let mut result : Vec<i8> = Vec::new();

    for _ in 0..case {
        let mut bf = String::new();
        std::io::stdin().read_line(&mut bf).expect("Err");
        let xyr : Vec<i32> = bf.split_whitespace()
                                .map(|s|s.trim().parse::<i32>().expect("Err"))
                                .collect();
        if !(xyr.len() < 6) {
            let r1 = xyr[2]; let r2 = xyr[5];
            let rad_max = r1.max(r2) as f32;
            let rad_min = r1.min(r2) as f32;
            let dst = (((xyr[3] - xyr[0]) * (xyr[3] - xyr[0])) as f32 + ((xyr[4] - xyr[1]) * (xyr[4] - xyr[1])) as f32).sqrt();

            if dst == 0 as f32 && rad_max == rad_min {result.push(-1)}
            else if dst == 0 as f32 && rad_max != rad_min {result.push(0)}
            else if dst > (rad_max + rad_min) || dst < (rad_max - rad_min) {result.push(0)}
            else if dst == (rad_max + rad_min) || dst == (rad_max - rad_min) {result.push(1)}
            else if dst < (rad_max + rad_min) {result.push(2)}
            else { println!("Err")}

        } else { panic!{"out of bound"} }
    }

    for i in 0..result.len() {
        println!("{}", result[i]);
    }
}