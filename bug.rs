fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut i = 0;

    loop {
        if i >= numbers.len() {
            break;
        }

        if numbers[i] % 2 == 0 {
            numbers.remove(i);
        } else {
            i += 1;
        }
    }

    println!("Result: {:?}", numbers);
}