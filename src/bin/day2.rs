
struct Report {
    levels: Vec<u32>,
    ascending: bool
}

impl Report {
    fn is_safe(&self) -> bool {
        if self.ascending {
            self.levels.windows(2).map(|it| (it[0], it[1])).all(|(a,b)| {
                a > b && (1..=3).contains(&(a as i32 - b as i32).abs())
            })
        } else {
            self.levels.windows(2).map(|it| (it[0], it[1])).all(|(a,b)| {
                a < b && (1..=3).contains(&(a as i32 - b as i32).abs())
            })
        }
    }

    fn is_semi_safe(&self) -> bool {
        if self.is_safe() { return true} //it allready has no error
        for i in 0..self.levels.len() {
            let mut fix = self.levels.clone();
            fix.remove(i);
            if new_report(fix).is_safe() {return true}
        }
        false
    }
}

fn new_report(levels: Vec<u32>) -> Report {
    Report {
        ascending: levels[0] >= levels[1],
        levels
    }
}

fn main() {
    let (safe,semi): (Vec<bool>, Vec<bool>) = include_str!("day2.txt").lines().map(|it| {
        new_report(it.split_whitespace().map(|it| it.parse::<u32>().unwrap()).collect::<Vec<u32>>())
    }).map(|x | (x.is_safe(), x.is_semi_safe())).unzip();
    println!("safe      reports: {}", safe.iter().filter(|x| **x).count());
    println!("semi-safe reports: {}", semi.iter().filter(|x| **x).count());
}