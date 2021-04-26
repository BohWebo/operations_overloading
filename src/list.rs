use std::ops::{Add, Sub, Div, Mul, Rem, Deref, DerefMut};
use crate::num::{Num};

enum Operations {
    Add,
    Subtract,
    Multiply,
    Division,
    Remainder,
}

/*TODO:
    ?add the ability to do bitwise operations
    ?add the ability to compare two lists
*/
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

impl<'self, T: Num> Deref for List<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &'self Self::Target {
        &self.storage
    }
}

impl<'self, T: Num> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &'self mut Self::Target {
        &mut self.storage
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

impl From<Vec<u8>> for List<u8> {
    fn from(storage: Vec<u8>) -> Self {
        List { storage }
    }
}

impl From<Vec<u16>> for List<u16> {
    fn from(storage: Vec<u16>) -> Self {
        List { storage }
    }
}

impl From<Vec<i16>> for List<i16> {
    fn from(storage: Vec<i16>) -> Self {
        List { storage }
    }
}

impl From<Vec<u32>> for List<u32> {
    fn from(storage: Vec<u32>) -> Self {
        List { storage }
    }
}

impl From<Vec<i32>> for List<i32> {
    fn from(storage: Vec<i32>) -> Self {
        List { storage }
    }
}

impl From<Vec<u64>> for List<u64> {
    fn from(storage: Vec<u64>) -> Self {
        List { storage }
    }
}

impl From<Vec<i64>> for List<i64> {
    fn from(storage: Vec<i64>) -> Self {
        List { storage }
    }
}

impl From<Vec<f32>> for List<f32> {
    fn from(storage: Vec<f32>) -> Self {
        List { storage }
    }
}

impl From<Vec<f64>> for List<f64> {
    fn from(storage: Vec<f64>) -> Self {
        List { storage }
    }
}


impl From<List<u8>> for Vec<u8> {
    fn from(list: List<u8>) -> Self {
        list.storage
    }
}

impl From<List<u16>> for Vec<u16> {
    fn from(list: List<u16>) -> Self {
        list.storage
    }
}

impl From<List<i16>> for Vec<i16> {
    fn from(list: List<i16>) -> Self {
        list.storage
    }
}

impl From<List<u32>> for Vec<u32> {
    fn from(list: List<u32>) -> Self {
        list.storage
    }
}

impl From<List<i32>> for Vec<i32> {
    fn from(list: List<i32>) -> Self {
        list.storage
    }
}

impl From<List<u64>> for Vec<u64> {
    fn from(list: List<u64>) -> Self {
        list.storage
    }
}

impl From<List<i64>> for Vec<i64> {
    fn from(list: List<i64>) -> Self {
        list.storage
    }
}

impl From<List<f32>> for Vec<f32> {
    fn from(list: List<f32>) -> Self {
        list.storage
    }
}

impl From<List<f64>> for Vec<f64> {
    fn from(list: List<f64>) -> Self {
        list.storage
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