use std::io::{Write, BufWriter, self};

fn dfs(graph : Vec<Vec<usize>>, mut visited : Vec<Vec<bool>>, size : (usize, usize)) {
    
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut tmp = Vec::new();
    let mut cnt = 0;
    for i in 0..size.0 {
        for j in 0..size.1 {
            if graph[i][j] == 1 && !visited[i][j] {
                tmp.push((i,j));
                while !tmp.is_empty() {
                    let current = tmp.pop().unwrap();
                    let x = current.0; let y = current.1;
                    visited[x][y] = true;
            
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
                        } else if x == size.0 - 1 && y == size.1 - 1 {
                            if h == 1 || w == 1 {
                                continue;
                            }
                        } else if x == size.0 - 1 {
                            if w == 1 {
                                continue;
                            }
                        } else if y == size.1 - 1 {
                            if h == 1 {
                                continue;
                            }
                        }
            
                        let nx = (x as isize + w) as usize;
                        let ny = (y as isize + h) as usize;
            
                        if nx < size.0 && ny < size.1 && graph[nx][ny] == 1 && !visited[nx][ny] {
                            tmp.push((nx, ny));
                        }
                    }
                } cnt += 1;
            } 
        }
    } write!(out, "{}\n", cnt).unwrap();
}

fn main() {

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let case = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    for _ in 0..case {
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let w = iter.next().unwrap().trim().parse::<usize>().unwrap();
        let h = iter.next().unwrap().trim().parse::<usize>().unwrap();
        let cb = iter.next().unwrap().trim().parse::<usize>().unwrap();
        buf.clear();

        let mut f = vec![vec![0; h]; w];
        let visit = vec![vec![false; h]; w];

        for _ in 0..cb {
            stdin.read_line(&mut buf).unwrap();
            let mut iter = buf.split_whitespace();
            let x = iter.next().unwrap().trim().parse::<usize>().unwrap();
            let y = iter.next().unwrap().trim().parse::<usize>().unwrap();
            f[x][y] = 1; buf.clear();
        }

        dfs(f, visit, (w, h));
    }
}
