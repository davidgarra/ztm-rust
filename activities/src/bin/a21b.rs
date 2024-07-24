// Topic: Map combinator
//
// Requirements:
// * Given a list of integers, use the map combinator to create a new list where each number is doubled.
// * Filter out any numbers greater than 10 from this new list.
// * Sum the remaining numbers and print the result.
//
// Notes:
// * Start by creating a vector of integers.
// * Use the iter, map, filter, and sum methods to achieve the desired functionality.
// * Make sure to print out the final sum.

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = v.iter().map(|n| n * 2).filter(|n| n <= &10).sum::<i32>();
    println!("The sum is {sum}");
}
