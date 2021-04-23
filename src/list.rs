use std::ops::{Add, Sub, Div, Mul, Rem};
use crate::num::{Num};

enum Operations {
    Add,
    Subtract,
    Multiply,
    Division,
    Remainder,
}

#[derive(Debug)]
pub struct List<T: Num> {
    storage: Vec<T>,
}

impl<T: Num> List<T> {
    pub fn new(storage: Vec<T>) -> Self {
        List {
            storage
        }
    }
}

impl<T: Num> Add for &List<T> {
    type Output = List<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let storage = elementwise_operation(&self.storage, &rhs.storage, Operations::Add);

        List { storage }
    }
}

impl<T: Num> Sub for &List<T> {
    type Output = List<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let storage = elementwise_operation(&self.storage, &rhs.storage, Operations::Subtract);

        List { storage }
    }
}

impl<T: Num> Div for &List<T> {
    type Output = List<T>;

    fn div(self, rhs: Self) -> Self::Output {
        let storage = elementwise_operation(&self.storage, &rhs.storage, Operations::Division);

        List { storage }
    }
}

impl<T: Num> Mul for &List<T> {
    type Output = List<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let storage = elementwise_operation(&self.storage, &rhs.storage, Operations::Multiply);

        List { storage }
    }
}

impl<T: Num> Rem for &List<T> {
    type Output = List<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        let storage = elementwise_operation(&self.storage, &rhs.storage, Operations::Remainder);

        List { storage }
    }
}

fn do_operation<T: Num>(x: T, y: T, op: &Operations) -> T {
    match op {
        &Operations::Add => x + y,
        &Operations::Subtract => x - y,
        &Operations::Multiply => x * y,
        &Operations::Division => x / y,
        &Operations::Remainder => x % y,
    }
}

fn elementwise_operation<T: Num>(vec1: &Vec<T>, vec2: &Vec<T>, operation: Operations) -> Vec<T> {
    if vec1.len() == vec2.len() {
        let mut output = Vec::with_capacity(vec1.capacity());

        for (index, item) in vec1.iter().enumerate() {
            let item1 = *item;
            let item2 = vec2[index];

            let result = do_operation(item1, item2, &operation);

            output.push(result);
        }

        output
    } else {
        let (smaller_vec, largest_vec) = if vec1.len() > vec2.len() {
            (vec2, vec1)
        } else {
            (vec1, vec2)
        };


        let mut output = Vec::with_capacity(largest_vec.capacity());
        let mut smaller_vec_index: usize = 0;

        for item in largest_vec.iter() {
            let item1 = *item;
            let item2 = smaller_vec[smaller_vec_index];

            let result = do_operation(item1, item2, &operation);

            output.push(result);

            if smaller_vec_index == smaller_vec.len() - 1 {
                smaller_vec_index = 0;
            } else {
                smaller_vec_index += 1;
            }
        }

        output
    }
}