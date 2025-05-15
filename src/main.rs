use merge_intervals::*;
use three_digit_even_numbers::*;
use two_sum::*;

mod merge_intervals;
mod three_digit_even_numbers;
mod two_sum;

fn main() {
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
    println!("{:?}", two_sum(vec![3, 3], 6));
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));

    let start = std::time::Instant::now();
    println!("{:?}", three_digit_even_numbers(vec![2, 1, 3, 0]));
    println!("duration: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("{:?}", three_digit_even_numbers(vec![2, 2, 8, 8, 2]));
    println!("duration: {:?}", start.elapsed());

    println!("{:?}", insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]));
    println!(
        "{:?}",
        insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ],
            vec![4, 8]
        )
    );
}
