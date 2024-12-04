use regex::Regex as Re;

#[derive(PartialEq,Eq)]
enum Token {
    Mul(usize,u32,u32),
    Do(usize),
    Dont(usize),
}

impl Token {
    fn get_idx(&self) -> usize {
        match self {
            Token::Mul(x,_,_) => *x,
            Token::Do(x) => *x,
            Token::Dont(x) => *x,
        }
    }
}

impl std::cmp::PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Token {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs = self.get_idx();
        let rhs = other.get_idx();
        lhs.cmp(&rhs)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let haystack = include_str!("day3.txt");
    let mut mul: Vec<Token> = Re::new("mul\\((\\d+),(\\d+)\\)")?.captures_iter(haystack).map(|x| {
        let (_,lr) = x.extract::<2>();
        let start = x.get(0).unwrap().start();
        Token::Mul(start, lr[0].parse::<u32>().unwrap(), lr[1].parse::<u32>().unwrap())
    }).collect();

    mul.append(&mut Re::new("do\\(\\)")?.captures_iter(haystack).map(|x| {
        Token::Do(x.get(0).unwrap().start())
    }).collect());

    mul.append(&mut Re::new("don't\\(\\)")?.captures_iter(haystack).map(|x| {
        Token::Dont(x.get(0).unwrap().start())
    }).collect());
    

    mul.sort();

    let mut enable = true;
    let mut accu: u32 = 0; 
    for token in mul {
        match token {
            Token::Mul(_, l, r) => if enable {
                accu += l*r
            },
            Token::Do(_) => enable = true,
            Token::Dont(_) => enable = false,
        }
    }

    println!("accu: {accu}");
    Ok(())
}
