fn main() {
    let mut triangles : Vec<[usize; 3]> = Vec::new();
    loop {
        let mut bf = String::new();
        std::io::stdin().read_line(&mut bf).expect("Err");
        let temp : Vec<usize> = bf.split_whitespace()
                                   .map(|s|s.trim().parse().expect("Err"))
                                   .collect::<Vec<_>>();
        
        if !(temp.len() < 3){
            if temp[0] == 0 && temp[1] == 0 && temp[2] == 0 {
                break;
            } else {
                triangles.push([temp[0], temp[1], temp[2]]);
            }
        } else { panic!("Out of Bound"); }
    }

    for i in 0..triangles.len() {
        let mut triangle = triangles[i];
        if triangle[0] == 0 || triangle[1] == 0 || triangle[2] == 0{
            println!("wrong"); continue;
        } else {
            triangle.sort();
            if triangle[2]*triangle[2] == triangle[0]*triangle[0] + triangle[1]*triangle[1] {
                println!("right");
            } else { println!("wrong"); }
        }
    }
}