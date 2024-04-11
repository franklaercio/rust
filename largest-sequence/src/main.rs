fn longest_sequence(numbers: &[i32]) -> &[i32] {
    let mut current_sequence = if !numbers.is_empty() { &numbers[0..1] } else { &[] };
    let mut longest_sequence = if !numbers.is_empty() { &numbers[0..1] } else { &[] };

    for i in 1..numbers.len() {
        if &numbers[i] > &numbers[i - 1] {
            current_sequence = &numbers[i - current_sequence.len()..=i];
        } else {
            current_sequence = &numbers[i..=i];
        }

        if current_sequence.len() > longest_sequence.len() {
            longest_sequence = current_sequence;
        }
    }

    return longest_sequence;
}

fn main() {
    assert_eq!(longest_sequence(&[1, 2, 3, 4, 5]), &[1, 2, 3, 4, 5]);
    assert_eq!(longest_sequence(&[5, 4, 3, 2, 1]), &[5]);
    assert_eq!(longest_sequence(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]), &[1, 5, 9]);
    assert_eq!(longest_sequence(&[3, 3, 3, 1, 2, 3, 4, 5]), &[1, 2, 3, 4, 5]);
    assert_eq!(longest_sequence(&[42]), &[42]);
    assert_eq!(longest_sequence(&[]), &[]);
    assert_eq!(longest_sequence(&[-5, -3, -1, 0, 2, 4, 6, 8]), &[-5, -3, -1, 0, 2, 4, 6, 8]);
    assert_eq!(longest_sequence(&[1, 2, 3, 3, 4, 5]), &[1, 2, 3]);
    assert_eq!(longest_sequence(&[10, 5, 8, 2, 3, 7, 1, 4, 6, 9]), &[1, 4, 6, 9]);
    assert_eq!(longest_sequence(&[3, 5, 1, 6, 2, 4, 8, 9, 7]), &[2, 4, 8, 9]);

    println!("All tests passed with success!");
}
