use std::io;

fn main() {
    let _ = input();
    let numbers = input();

    let numbers: Vec<&str> = numbers.trim().split(' ').collect();
    let mut answers: Vec<String> = Vec::with_capacity(numbers.len());
    for number in numbers {
        let answer = calc_answer(number.parse::<i32>().unwrap());
        answers.push(answer.to_string());
    }
    println!("{}", answers.join("\n"));
}

fn calc_answer(t: i32) -> i32 {
    multiple_sum(3, t) + multiple_sum(7, t) - multiple_sum(21, t)
}

fn sum(to: i32) -> i32 {
    to * (to + 1) / 2
}

fn multiple_sum(n: i32, to: i32) -> i32 {
    n * sum(to / n)
}

fn input() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .ok();

    return line;
}