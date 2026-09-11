fn add<'a>(a: &'a i32, b: &'a i32) -> i32 {
    a + b
}

fn main() {
    let value = add(&4, &4);
    println!("{}", value)
}
