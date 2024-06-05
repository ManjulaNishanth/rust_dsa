use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn exec_empty_vector() -> Vec<i32> {
    let mut data_vec = Vec::new();

    data_vec.push(1);
    data_vec.push(2);
    data_vec.push(3);

    match data_vec.pop() {
        Some(data) => {
            println!("pop {:?}", data)
        }
        None => println!("None"),
    }

    println!("\n data_vec {:?} ", &data_vec);
    data_vec
}

pub fn exec_vector() -> Vec<i32> {
    let mut data_vec = vec![9, 8, 7];
    match data_vec.pop() {
        Some(data) => {
            println!("pop {:?}", data)
        }
        None => println!("None"),
    }

    println!("\n data_vec {:?} ", &data_vec);
    data_vec
}

/*
Two Sum using Naive Approach:
The basic approach to solve this problem is by nested traversal.

Traverse the array using a loop
For each element:
    Check if there exists another in the array with sum as x
    Return true if yes, else continue
If no such pair is found, return false.
*/
pub fn check_sum_exist() -> String {
    let mut data_vec = vec![0, -1, -1, 2, -3, 1];
    let sum = -2;

    for data in 0..(data_vec.len() - 1) {
        // println!(" outer len {:?} - {:?}", data, data_vec[data]);
        for inner_data in (data + 1)..data_vec.len() {
            // println!(" inner len {:?} - {:?}", inner_data, data_vec[inner_data]);
            match data_vec[data] + data_vec[inner_data] == sum {
                true => {
                    println!(
                        " Sum of {:?} + {:?} is Equalen {:?}",
                        data_vec[data], data_vec[inner_data], sum
                    )
                }
                false => {
                    println!(
                        " Sum of {:?} + {:?} is Not Equalen {:?}",
                        data_vec[data], data_vec[inner_data], sum
                    )
                }
            }
        }
    }
    "ok".to_string()
}

/*
Given an array of n elements that contains elements from 0 to n-1, with any of these numbers appearing any number of times. Find these repeating numbers in O(n) and use only constant memory space.
Note: The repeating element should be printed only once.
*/

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum DuplicateResult {
    NotFound(i32),
    Found(Vec<i32>),
}

pub fn find_duplicates() -> Value {
    let data_vec = vec![0, -1, -1, 2, -3, 1];
    let mut new_hashmap: HashMap<i32, i32> = HashMap::new();
    let mut result_vec: Vec<i32> = Vec::new();

    for vote in data_vec {
        new_hashmap
            .entry(vote)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    println!("new_hashmap : {new_hashmap:#?}");

    for (key, value) in new_hashmap.iter() {
        println!("value : {value:#?}");
        if *value > 1 {
            result_vec.push(*key)
        }
    }
    println!("result_vec : {result_vec:#?}");
    let y = json!(DuplicateResult::Found(result_vec.clone()));

    if result_vec.len() > 0 {
        json!(DuplicateResult::Found(result_vec))
    } else {
        json!(DuplicateResult::NotFound(-1))
    }
}
