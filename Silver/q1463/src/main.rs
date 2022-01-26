use std::io;
use std::cmp;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no Input.");
    let num : usize = text.trim().parse().expect("Input Error.");
    let mut depth = vec![0; 1000001];

    depth[1] = 0;
    for i in 2..1000001 {
        if i % 6 == 0 {
            depth[i] = cmp::min(cmp::min(depth[i/3]+1, depth[i/2]+1), depth[i-1]+1);
        } else if i % 3 == 0 {
            depth[i] = cmp::min(depth[i/3]+1, depth[i-1]+1);
        } else if i % 2 == 0 {
            depth[i] = cmp::min(depth[i/2]+1, depth[i-1]+1);
        } else {
            depth[i] = depth[i-1] + 1;
        }
    }

    print!("{}", depth[num]);
}
