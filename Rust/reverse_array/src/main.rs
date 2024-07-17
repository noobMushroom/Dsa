fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", numbers);
    reverse(&mut numbers);
    println!("{:?}", numbers);
}

fn reverse(arr: &mut Vec<u32>) {
    let mut start = 0;
    let mut last = arr.len() - 1;

    while start < last {
        let temp = arr[start];
        arr[start] = arr[last];
        arr[last] = temp;
        start += 1;
        last -= 1;
    }
}
