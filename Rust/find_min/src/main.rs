fn main() {
    let numbers = vec![12, 3, 4, 3, 53, 8, 03, 3];
}

fn find_min(numbers: &Vec<u32>) -> u32 {
    let mut min = numbers[0];

    for i in numbers {
        if min > *i {
            min = *i
        }
    }

    // short way will be
    // numbers.iter().min();

    min
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn output_right() {
        let numbers = vec![12, 3, 4, 3, 53, 8, 03, 3];
        assert_eq!(find_min(&numbers), 3);
    }
}
