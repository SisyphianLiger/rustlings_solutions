// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.


fn main() {
    let answer:i32 = square(3);
    println!("The square of 3 is {}", answer);
    let answer_ret:i32 = ret_square(3);
    println!("The square of 3 is {}", answer_ret);
}

fn square(num: i32) -> i32 {
    num * num
}

fn ret_square(num: i32) -> i32 {
    return num * num;
}
