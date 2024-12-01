use std::collections::{BTreeMap, VecDeque};


fn main() {
    let input = include_str!("day1.txt");
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();

    input.lines().for_each(|line| {
        let mut parts: Vec<&str> = line.split("   ").collect();
        left.push_front(parts.pop().unwrap().parse::<u32>().unwrap());
        right.push_front(parts.pop().unwrap().parse::<u32>().unwrap());
    });

    let mut seen: BTreeMap<u32, u32> = BTreeMap::new();

    let sum: u32 = left.iter().map(|num| {
        if let Some(val) = seen.get(num) {
            num * val
        } else {
            let rcount: u32 = right.iter().map(|rnum| {
                if rnum == num {1} else {0}
            }).sum();
            seen.insert(*num, rcount);
            num * rcount
        }
    }).sum();
    println!("result: {}",sum)

}