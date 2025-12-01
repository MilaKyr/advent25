use crate::days::utils::show_answer;
const MAX_NUMBER: i32 = 100;
pub fn run() {
    let mut start = 50;
    let mut ans1 = 0;
    let mut ans2 = 0;

    include_str!("../../data/day1.txt").lines().for_each(|line| {
        let (dir, number_s) = line.split_at(1);
        let mut num = number_s.parse::<i32>().unwrap();
        ans2 += num / MAX_NUMBER;
        num %= MAX_NUMBER;
        start = if dir == "L" {
            if start < num && start != 0 {
                ans2 += 1;
            }
            (MAX_NUMBER + start - num) % MAX_NUMBER
        } else {
            if start + num > MAX_NUMBER {
                ans2 += 1;
            }
            (MAX_NUMBER + start + num) % MAX_NUMBER
        };
        if start == 0 {
            ans1 += 1;
            ans2 += 1;
        }

    });
    show_answer(1, ans1, ans2);
}

