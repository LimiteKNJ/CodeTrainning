use std::collections::VecDeque;

fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let times = bf.trim().parse::<usize>().expect("Err");
    let mut result : Vec<String> = Vec::new();

    for _ in 0..times {
        let mut bf = String::new();
        std::io::stdin().read_line(&mut bf).expect("Err");
        let comm : VecDeque<char> = bf.trim().chars().collect();
    
        let mut bf = String::new();
        std::io::stdin().read_line(&mut bf).expect("Err");
        let size = bf.trim().parse::<usize>().expect("Err");

        let mut bf = String::new();
        std::io::stdin().read_line(&mut bf).expect("Err");
        let matrix : VecDeque<char> = bf.trim().chars().collect();

        let mut matrix_data : VecDeque<u32> = VecDeque::new();
        let mut cnt = 0;
        let mut err = false;
        
        if matrix[0] != '[' || matrix[matrix.len()-1] != ']' {
            result.push("error".to_string()); continue;
        } else {
            let mut num : String = String::new();
            let mut num_d = 0;
            for i in 1..matrix.len() {
                if matrix[i].is_numeric() {
//                  '1''2'',''2''3'
                    if num_d == 0 {
                        num = matrix[i].to_string();
                        num_d += 1;
                    } else {
                        num.push(matrix[i]);
                    }
                } else {
                    num_d = 0;
                    if size == 0 {
                        
                    } else if cnt < size {
                        matrix_data.push_back(num.parse().expect("Err"));
                        num.clear();
                        cnt += 1;
                    } else {
                        err = true; break;
                    }
                }
            }

            let mut reverse_cnt = 0;
            for i in 0..comm.len(){
                if comm[i].is_alphabetic() {
                    if comm[i].to_ascii_uppercase() == 'R'  {
                        reverse_cnt += 1;
                    } else if comm[i].to_ascii_uppercase() == 'D' {
                        if !(matrix_data.is_empty()) {
                            if reverse_cnt % 2 == 1 {
                                matrix_data.pop_back();
                            } else {
                                matrix_data.pop_front();
                            }
                        } else { err = true; break; }
                    } else {
                        err = true; break;
                    }
                } else {
                    err = true; break;
                }
            }

            if reverse_cnt % 2 == 1 {
                matrix_data.make_contiguous().reverse();
            }

            if err { result.push("error\n".to_string()); }
            else { 
                result.push('['.to_string());
                for i in 0..matrix_data.len() {
                    if i == matrix_data.len() - 1 {
                        result.push(matrix_data[i].to_string());
                    } else {
                        result.push(matrix_data[i].to_string());
                        result.push(','.to_string());
                    }
                } result.push(']'.to_string());
                result.push('\n'.to_string());
             }
        }
    }

    for i in 0..result.len(){
        print!("{}", result[i]);
    }
}