use std::collections::VecDeque;

fn main() {
    let input = include_str!("day1.txt");
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();

    input.lines().for_each(|line| {
        let mut parts: Vec<&str> = line.split("   ").collect();
        left.push_front(parts.pop().unwrap().parse::<i32>().unwrap());
        right.push_front(parts.pop().unwrap().parse::<i32>().unwrap());
    });

    left.make_contiguous().sort();
    right.make_contiguous().sort();

    let smallest: i32 = left.iter().enumerate().map(|(i,v)| {
        (right[i] - v).abs()
    }).sum();


    println!("smallest: {}",smallest);
}
