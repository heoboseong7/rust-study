fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let add_3 = |x: i32| x + 3;
    print_type_of(&add_3);
    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("add_3: {}", apply_with_log(add_3, 20));

    let mut v = Vec::new();
    let mut accumulate = |x: i32| {
        v.push(x);
        v.iter().sum::<i32>()
    };
    print_type_of(&accumulate);
    println!("accumulate: {}", apply_with_log(&mut accumulate, 4));
    // Case 0
    // let mut val = 3
    // val = 5

    // Case 1
    // accumulate = |x: i32| {
    //     v.push(x);
    //     println!("{v:?}");
    //     v.iter().sum::<i32>()
    // };

    // Case 2
    // let mut accumulate = |x: i32| {
    //     v.push(x);
    //     println!("{v:?}");
    //     v.iter().sum::<i32>()
    // };

    // Case 3
    // accumulate = accumulate;

    println!("accumulate: {}", apply_with_log(&mut accumulate, 5));

    let multiply_sum = |x| x * v.into_iter().sum::<i32>();
    println!("multiply_sum: {}", apply_with_log(multiply_sum, 3));
}