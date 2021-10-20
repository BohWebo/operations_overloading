mod num;
mod list;
mod utils;
mod test_list;

use crate::list::List;

fn main() {
    let list1 = List::new(vec![1, 2, 3, 4, 5, 7, 9, 12]);
    let list2 = List::from(vec![1, 2, 3, 4, 0]);

    //TODO: write tests for all types of numbers.
    //TODO: cover possible overflowing
    let sum = &list1 + &list2;
    let difference = &list1 - &list2;
    let product = &list1 * &list2;
    let quotient = &list1 / &list2;
    let remainder = &list1 % &list2;

    let sum_vec: Vec<i32> = sum.into();

    println!("sum_vec {:?}", sum_vec);
    println!("difference {:?}", difference);
    println!("product {:?}", product);
    println!("quotient {:?}", quotient);
    println!("remainder {:?}", remainder);
}
