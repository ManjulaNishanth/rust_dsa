mod vector;

use crate::vector::vector::{check_sum_exist, exec_vector, find_duplicates};

fn main() {
    println!("Hello, world!");
    // exec_vector();
    // check_sum_exist();
    let result = find_duplicates();

    println!("find_duplicates : {result:?}");
}
