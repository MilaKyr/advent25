use crate::days::utils::show_answer;

fn repeated_number(mut number: i64) -> bool {
    let mut d = 1_i64;
    let mut part = number % 10;
    number /= 10;
    while part == 0 {
        d = d * 10;
        number /= 10;
        part = number % 10;
    }

    d = if d == 1 { 10} else {d};



    while part % d != number % d {
        part = (number % d) * 10 + part;
        number /= d;
        d *= 10;
        println!("{d}, {number}, {part} - {}, {}", part % d, number % d);
    }
    // println!("2  {d}, {number}, {part}");

    if number  == 0 {
        return false;
    }
    if part == 0 {
        return false;
    }
    // println!("3 {part} {number} {d} {}", number % d);
    while number > 0 {
        if number % d != part {
            return false;
        }
        number /= d;
    }
    true
}
pub fn run() {
    let mut ans1 = 0;
    let mut ans2 = 0;

    include_str!("../../data/day2.txt").split_terminator(",").for_each(|r| {
        let mut splitter = r.split_terminator("-");
        let start = splitter.next().unwrap().parse::<i64>().unwrap();
        let end = splitter.next().unwrap().parse::<i64>().unwrap();
        for val in start..=end {
            let n_digits = val.ilog(10) + 1;
            if n_digits % 2 == 0 {
                let denominator = 10_i64.pow(n_digits / 2);
                if val % denominator == val / denominator {
                    ans1 += val;
                } else if repeated_number(val) {
                    ans2 += val;
                }
            } else if repeated_number(val) {
                ans2 += val;
            }
        }
    });

    show_answer(2, ans1, ans2);
}

