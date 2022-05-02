use std::io::{Write, BufWriter, self};
use std::collections::VecDeque;

fn print(cnt : usize, arrive : bool){
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    if arrive {
        write!(out, "{}\n", cnt).unwrap();
    } else {
        write!(out, "-1").unwrap();
    }
}

fn bfs(graph : Vec<Vec<char>>, mut visited : Vec<Vec<Vec<bool>>>, max_x : usize, max_y : usize) {
    
    let mut tmp = VecDeque::new();
    visited[0][0][0] = true;
    tmp.push_back((0,0,0,1));

    while !tmp.is_empty() {
        let current = tmp.pop_front().unwrap();
        let x = current.0; let y = current.1;
        let wall = current.2; let cnt = current.3;

        if x == max_x - 1 && y == max_y - 1 {
            print(cnt, true); return;

        } else {
            for i in vec![(1,0),(0,1),(-1,0),(0,-1)]{
                let m : isize = i.0; let n : isize= i.1;

                if x == 0 && y == 0 {
                    if m == -1 || n == -1 {
                        continue;
                    }
                } else if x == 0 {
                    if m == -1 {
                        continue;
                    }
                } else if y == 0 {
                    if n == -1 {
                        continue;
                    }
                }
                
                let next_x = (x as isize + m) as usize;
                let next_y = (y as isize + n) as usize;

                if next_x < max_x && next_y < max_y && graph[next_x][next_y] == '0'{
                    if !visited[next_x][next_y][wall] {
                        tmp.push_back((next_x, next_y, wall, cnt+1));
                        visited[next_x][next_y][wall] = true;
                    }
                } else if next_x < max_x && next_y < max_y && graph[next_x][next_y] == '1' && wall == 0 {
                    if !visited[next_x][next_y][wall] {
                        tmp.push_back((next_x, next_y, wall+1, cnt+1));
                        visited[next_x][next_y][wall+1] = true;
                    }
                }
            }
        }
    } print(0, false);
} 

fn main() {
     
    let stdin = io::stdin();
 
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let x = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let y = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    let mut maze = Vec::new();
    let visited = vec![vec![vec![false; 2] ; y]; x];
    for _ in 0..x {
        stdin.read_line(&mut buf).unwrap();
        let data = buf.trim().chars().collect::<Vec<_>>();
        maze.push(data);
        buf.clear();
    }

    bfs(maze, visited, x, y);
}