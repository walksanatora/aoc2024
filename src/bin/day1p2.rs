use std::collections::{BTreeMap, VecDeque};


fn main() {
    print!("starting");
    let input = include_str!("day1.txt");
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();

    let mut l_count = [0u32; 99999];
    let mut r_count = [0u32; 99999];

    print!("made data");
    input.lines().for_each(|line| {
        let mut parts: Vec<&str> = line.split("   ").collect();
        let l = parts.pop().unwrap().parse::<u32>().unwrap();
        l_count[l as usize] += 1;
        left.push_front(l);
        let r = parts.pop().unwrap().parse::<u32>().unwrap();
        r_count[r as usize] += 1;
        right.push_front(r);
    });
    print!("data parsed");
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
    println!("result: {}",sum);

    let other_result: u32 = l_count.iter().zip(r_count).enumerate().map(|(i, (l, r))| {
        (i as  u32)*l*r
    }).sum();
    println!("other_result = {}", other_result);

}