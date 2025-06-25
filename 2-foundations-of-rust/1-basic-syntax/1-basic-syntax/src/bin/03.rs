fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut big_int = input[0];
    let mut small_int = input[0];

    for num in input {
        if num > big_int {
            big_int = num
        } else if num < small_int {
            small_int = num
        }
    }

    println!("{big_int} is largest and {small_int} is smallest");
}
