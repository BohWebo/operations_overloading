use crate::num::Num;
pub enum Operations {
    Add,
    Subtract,
    Multiply,
    Division,
    Remainder
}

pub fn elementwise_operation<T: Num>(vec1: &Vec<T>, vec2: &Vec<T>, operation: Operations) -> Vec<T> {
    if vec1.len() == vec2.len() {
        elementwise_operation_by_equal_vectors(vec1, vec2, operation)
    } else {
        elementwise_operation_by_not_equal_vectors(vec1, vec2, operation)
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


fn get_largest_vector<'a, T: Num>(vec1: &'a Vec<T>, vec2: &'a Vec<T>) -> (&'a Vec<T>, &'a Vec<T>) {
    if vec1.len() > vec2.len() {
        (vec2, vec1)
    } else {
        (vec1, vec2)
    }
}

fn elementwise_operation_by_equal_vectors<T: Num> (vec1: &Vec<T>, vec2: &Vec<T>, operation: Operations) -> Vec<T> {
    let mut output = Vec::with_capacity(vec1.capacity());

    for (index, item) in vec1.iter().enumerate() {
        let item1 = *item;
        let item2 = vec2[index];

        let result = do_operation(item1, item2, &operation);

        output.push(result);
    }

    output
}

fn elementwise_operation_by_not_equal_vectors<T: Num> (vec1: &Vec<T>, vec2: &Vec<T>, operation: Operations) -> Vec<T> {
    let (smaller_vec, largest_vec) = get_largest_vector(vec1, vec2);
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