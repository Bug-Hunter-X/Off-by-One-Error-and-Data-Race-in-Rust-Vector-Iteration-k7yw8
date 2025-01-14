fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    numbers.retain(|&x| x % 2 != 0);

    println!("Result: {:?}", numbers);
}