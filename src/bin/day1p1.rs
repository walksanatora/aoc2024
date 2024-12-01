fn main() {
    let input = include_str!("day1.txt");
    let mut left = vec![];
    let mut right = vec![];

    input.lines().for_each(|line| {
        let mut parts: Vec<&str> = line.split("   ").collect();
        left.push(parts.pop().unwrap().parse::<i32>().unwrap());
        right.push(parts.pop().unwrap().parse::<i32>().unwrap());
    });

    left.sort();
    right.sort();




    let smallest: i32 = left.iter().enumerate().map(|(i,v)| {
        (right[i] - v).abs()
    }).sum();


    println!("smallest: {}",smallest);
}
