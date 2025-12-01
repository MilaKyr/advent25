use std::fmt::Display;

pub fn show_answer<T: Display>(day: usize, p1: T, p2: T) {
    println!("Day {day}");
    let mut v = vec!['-'; 12];
    for i in 0..day {
        v[i] = '*';
    }
    println!("{}", v.into_iter().collect::<String>());
    println!("Part 1 = {p1}, \n Part 2 = {p2}");
}