use std::io;

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1; // Search for first occurrence on the left side
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
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

    println!("Enter the target number:");
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    let target: i32 = target.trim().parse().expect("Invalid number");

    match first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("The element {} is not found in the array", target),
    }
}

