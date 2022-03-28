fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Error");
    let rec : Vec<f32> = bf.split_whitespace()
                            .map(|s|s.trim().parse().expect("Error"))
                            .collect::<Vec<_>>();

    if !(rec.len() < 4) {
        let x = rec[0]; let y = rec[1];
        let w = rec[2]; let h = rec[3];

        if x < w/2f32 && y < h/2f32 {
            if x < y {
                print!("{}", x);
            } else { 
                print!("{}", y);
            }
        } else if x >= w/2f32 && y < h/2f32 {
            if w-x < y {
                print!("{}", w-x);
            } else { 
                print!("{}", y);
            }
        } else if x < w/2f32 && y >= h/2f32 {
            if x < h-y {
                print!("{}", x);
            } else { 
                print!("{}", h-y);
            }
        } else if x >= w/2f32 && y >= h/2f32 {
            if w-x < h-y {
                print!("{}", w-x);
            } else { 
                print!("{}", h-y);
            }
        } else {
            print!("Err");
        }
    }
}