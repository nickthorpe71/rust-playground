fn main() {
    let mut vec = Vec::new();
    vec.push(2);
    vec.push(2);

    println!("{}", square_sum(vec));
}

fn square_sum(vec: Vec<i32>) -> i32 {
    let mut result = 0;

    for num in &vec {
        result += num * num;
    }

    return result;
}
