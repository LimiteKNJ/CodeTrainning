use std::io::{Write, BufWriter, self};
use std::collections::VecDeque;

fn bfs(mut graph : Vec<Vec<isize>>, max_x : usize, max_y : usize) -> isize {

    let mut tmp = VecDeque::new();
    let mut max_cnt = 0;
    for i in 0..max_x {
        for j in 0..max_y {
            if graph[i][j] == 1 {
                tmp.push_back((i,j,0));
            }
        }
    }

    while !tmp.is_empty() {
        let current = tmp.pop_front().unwrap();
        let x = current.0; let y = current.1;
        let day = current.2;

        for next_vec in vec![(1,0),(0,1),(-1,0),(0,-1)]{
            let w = next_vec.0;
            let h = next_vec.1;

            if x == 0 && y == 0 {
                if h == -1 || w == -1 {
                    continue;
                }
            } else if x == 0 {
                if w == -1 {
                    continue;
                }
            } else if y == 0 {
                if h == -1 {
                    continue;
                }
            } else if x == max_x - 1 && y == max_y - 1 {
                if h == 1 || w == 1 {
                    continue;
                }
            } else if x == max_x - 1 {
                if w == 1 {
                    continue;
                }
            } else if y == max_y - 1 {
                if h == 1 {
                    continue;
                }
            }

            let nx = (x as isize + w) as usize;
            let ny = (y as isize + h) as usize;

            if nx < max_x && ny < max_y && graph[nx][ny] == 0 {
                tmp.push_back((nx, ny, day+1));
                graph[nx][ny] = 1;
            }
        }

        if day > max_cnt {
            max_cnt = day;
        }
    }

    let mut zero_cnt = 0;
    for i in 0..max_x {
        for j in 0..max_y {
            if graph[i][j] == 0 {
                zero_cnt += 1;
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
    buf.clear();

    let mut tomato = Vec::new();
    for _ in 0..x {
        stdin.read_line(&mut buf).unwrap();
        let data = buf.split_whitespace()
                                .map(|s|s.trim().parse::<isize>().unwrap())
                                .collect::<Vec<_>>();
        tomato.push(data);
        buf.clear();
    }

    let result = bfs(tomato, x, y);
    write!(out, "{}\n", result).unwrap();
}