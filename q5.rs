use std::io;

fn find_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 0 {
        let mid = n / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[n / 2] as f64
    }
}

fn main() {
    println!("Enter the sorted array elements separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid number"))
        .collect();

    let median = find_median(&arr);
    println!("The median of the array is: {}", median);
}
