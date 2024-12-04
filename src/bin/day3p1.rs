fn main() {
    let regex = regex::Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();
    println!("result: {}", regex.captures_iter(include_str!("day3.txt")).map(|x| {
        x.extract::<2>()
    }).map(|(_, l)| {
        (l[0],l[1])
    }).map(|(l,r)| {
        l.parse::<u32>().unwrap()*r.parse::<u32>().unwrap()
    }).sum::<u32>());
}