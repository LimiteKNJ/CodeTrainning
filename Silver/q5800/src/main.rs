fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let class_cnt = bf.trim().parse::<usize>().expect("Err");

    for class_num in 0..class_cnt {
        let mut bf = String::new();
        std::io::stdin().read_line(&mut bf).expect("Err");
        let temp = bf.split_whitespace()
                                .map(|s|s.trim().parse().expect("Err"))
                                .collect::<Vec<usize>>();
        
        let class_stu = temp[0];
        let mut class_score = Vec::new();
        for i in 1..temp.len() {
            class_score.push(temp[i]);
        }

        let mut sub_max = 0;
        class_score.sort_by(|a, b| a.cmp(b));
        for i in 0..class_stu-1 {
            let temp = class_score[i+1] - class_score[i];
            if sub_max < temp { sub_max = temp; }
            else { continue; }
        }

        println!("Class {}\nMax {}, Min {}, Largest gap {}",
                class_num + 1, class_score[class_score.len()-1], class_score[0], sub_max);
    }
}