use std::io;
use io::Write;

fn p_in_circle (point : (isize, isize), circle : Vec<isize>) -> bool {
    return circle[2] as f64 > (((point.0 - circle[0]) * (point.0 - circle[0])
    + (point.1 - circle[1]) * (point.1 - circle[1])) as f64).sqrt();
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let times = buf.trim().parse::<usize>().unwrap();

    for _ in 0..times {
        let stdout = std::io::stdout();
        let mut out = io::BufWriter::new(stdout.lock());

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p_buf = buf.split_whitespace()
                        .map(|s|s.trim().parse().unwrap())
                        .collect::<Vec<isize>>();

        let p_start = (p_buf[0],p_buf[1]);
        let p_end  = (p_buf[2],p_buf[3]);

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let planet_num = buf.trim().parse::<usize>().unwrap();

        let mut planets : Vec<Vec<isize>> = Vec::new();
        let mut cnt = 0;
        for _ in 0..planet_num {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            let planet = buf.split_whitespace()
                    .map(|s|s.trim().parse().unwrap())
                    .collect::<Vec<isize>>();
            planets.push(planet);
        }

        for iter_p in planets {
            if p_in_circle(p_start, iter_p.clone()) && p_in_circle(p_end, iter_p.clone()) {
                cnt += 0;

            } else if p_in_circle(p_start, iter_p.clone()) || p_in_circle(p_end, iter_p.clone()) {
                cnt += 1;
                
            } else {
                cnt += 0;

            }
        }

        write!(out, "{}\n", cnt);
    }
}