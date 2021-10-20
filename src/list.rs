use std::ops::{Add, Sub, Div, Mul, Rem, Deref, DerefMut};
use crate::num::{Num};
use crate::utils::{Operations, elementwise_operation};

/*TODO:
    ?add the ability to do bitwise operations
    ?add the ability to compare two lists (how do I compare two lists, by the sums of their elements?)
*/
#[derive(Debug)]
pub struct List<T: Num> {
    storage: Vec<T>
}

impl<T: Num> List<T> {
    pub fn new(storage: Vec<T>) -> Self {
        List {
            storage
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.storage
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.storage
    }
}

impl<T: Num> Deref for List<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl<T: Num> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
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

impl<T: Num> From<Vec<T>> for List<T> {
    fn from(storage: Vec<T>) -> Self {
        List { storage }
    }
}

impl<T: Num> From<List<T>> for Vec<T> {
    fn from(list: List<T>) -> Self { list.storage }
}
