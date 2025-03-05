use std::cmp::Ordering;

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }
    let (positives, negatives) = input
        .iter()
        .fold((0, 0), |(post_count, neg_sum), &num| match num.cmp(&0) {
            Ordering::Greater => (post_count + 1, neg_sum),
            Ordering::Less => (post_count, neg_sum + num),
            Ordering::Equal => (post_count, neg_sum),
        });

    vec![positives, negatives]
}

#[cfg(test)]
mod tests {
    use super::count_positives_sum_negatives;

    fn dotest(a: &[i32], expected: &[i32]) {
        let actual = count_positives_sum_negatives(a.to_vec());
        assert!(
            actual == expected,
            "With input = {a:?}\nExpected {expected:?} but got {actual:?}"
        )
    }
    #[test]
    fn fixed_tests() {
        dotest(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15],
            &[10, -65],
        );
        dotest(&[], &[]);
        dotest(
            &[0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14],
            &[8, -50],
        );
        dotest(&[0, 1, 2, 3, 4, 5], &[5, 0]);
        dotest(&[1, 2, 3, 4, 5], &[5, 0]);
        dotest(&[0, -1, -2, -3, -4, -5], &[0, -15]);
        dotest(&[-1, -2, -3, -4, -5], &[0, -15]);
        dotest(&[0, 0, 0, 0], &[0, 0]);
        dotest(&[0], &[0, 0]);
    }
}
