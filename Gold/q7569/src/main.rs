use std::io::{Write, BufWriter, self};
use std::collections::VecDeque;

fn bfs(mut graph : Vec<Vec<Vec<isize>>>, max_x : usize, max_y : usize, max_h : usize) -> isize {
    
    let mut tmp = VecDeque::new();
    let mut max_cnt = 0;
    for h in 0..max_h {
        for i in 0..max_x {
            for j in 0..max_y {
                if graph[h][i][j] == 1 {
                    tmp.push_back((i,j,h,0));
                }
            }
        }
    }

    while !tmp.is_empty() {
        let current = tmp.pop_front().unwrap();
        let x = current.0; let y = current.1; let h = current.2;
        let day = current.3;

        for next_vec in vec![(1,0,0),(0,1,0),(0,0,1),(-1,0,0),(0,-1,0),(0,0,-1)]{
            let w_box = next_vec.0;
            let h_box = next_vec.1;
            let _box = next_vec.2;

            if x == 0 && y == 0 && h == 0 {
                if h_box == -1 || w_box == -1 || _box == -1 {
                    continue;
                }
            } else if x == 0 {
                if w_box == -1 {
                    continue;
                }
            } else if y == 0 {
                if h_box == -1 {
                    continue;
                }
            } else if h == 0 {
                if _box == -1 {
                    continue;
                }
            } else if x == max_x - 1 && y == max_y - 1 && h == max_h - 1 {
                if h_box == 1 || w_box == 1 || _box == 1 {
                    continue;
                }
            } else if x == max_x - 1 {
                if w_box == 1 {
                    continue;
                }
            } else if y == max_y - 1 {
                if h_box == 1 {
                    continue;
                }
            } else if h == max_h - 1 {
                if _box == 1 {
                    continue;
                }
            }

            let nx = (x as isize + w_box) as usize;
            let ny = (y as isize + h_box) as usize;
            let nh = (h as isize + _box) as usize;

            if nx < max_x && ny < max_y && nh < max_h && graph[nh][nx][ny] == 0 {
                tmp.push_back((nx, ny, nh, day+1));
                graph[nh][nx][ny] = 1;
            }
        }

        if day > max_cnt {
            max_cnt = day;
        }
    }

    let mut zero_cnt = 0;
    for h in 0..max_h {
        for i in 0..max_x {
            for j in 0..max_y {
                if graph[h][i][j] == 0 {
                    zero_cnt += 1;
                }
            }
        }
    }

    if zero_cnt > 0 {
        return -1;
    } else {
        return max_cnt;
    }
}

fn main() {
     
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
 
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let y = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let x = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let h = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    let mut tomato_box = Vec::new();
    for _ in 0..h {
        let mut tomato = Vec::new();
        for _ in 0..x {
            stdin.read_line(&mut buf).unwrap();
            let data = buf.split_whitespace()
                                    .map(|s|s.trim().parse::<isize>().unwrap())
                                    .collect::<Vec<_>>();
            tomato.push(data);
            buf.clear();
        } tomato_box.push(tomato);
    }

    let result = bfs(tomato_box, x, y, h);
    write!(out, "{}\n", result).unwrap();
}