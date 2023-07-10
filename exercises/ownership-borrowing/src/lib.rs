#![allow(dead_code)]
#![allow(unused_variables)]
// Exercise 1
// Make it compile
// Cach 1
fn exercise1a() {
    let x = String::from("hello, world");
    let y = x.clone();
    let z = x.clone();
}
// Cach 2
fn exercise1b() {
    let x = String::from("hello, world");
    let y = &x;
    let z = &x;
}
// Cach 3
fn exercise1c() {
    let x = String::from("hello, world");
    let y = x;
    let z = y;
}

// Exercise 2
// Make it compile
// Don't modify code in exercise2 function!
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();

    let additions: Vec<usize> = vec![0];

    println!("{:?}", values_number);

    while !additions.is_empty() {
        let mut addition: f64 = 0.0;
        for &element_index in &additions {
            let addition_aux = values[element_index];
            addition = addition_aux + addition;
        }

        println!("{}", addition);
    }
}

// Exercise 4
// Make it compile
fn exercise4(value: u32) -> String {
    let str_value = value.to_string(); // Convert u32 to String
    str_value
}

// Exercise 5
// Make it compile
use std::collections::HashMap;
fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

    let key = 3;

    let res = match my_map.get(&key) {
        Some(child) => child.clone(),
        None => {
            let value = "3.0".to_string();
            my_map.insert(key, value.clone());
            value
        }
    };

    println!("{}", res);
}

// Exercise 6
// Make it compile

use std::io::{self, BufRead};

fn exercise6() {
    let mut prev_key: String = String::new();

    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(_) => {
                let s = line.trim();

                let data: Vec<&str> = s.split('\t').collect();
                if prev_key.is_empty() {
                    prev_key = data[0].to_string();
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}

// Exercise 7
// Make it compile
fn exercise7() {
    let mut v: Vec<String> = Vec::new();
    {
        let chars = [b'x', b'y', b'z'];
        let s: String = String::from_utf8(chars.to_vec()).unwrap();
        v.push(s);
    }
    println!("{:?}", v);
}


// Exercise 8
// Make it compile
fn exercise8() {
    let mut accounting = vec!["Alice".to_string(), "Ben".to_string()];

    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    for line_result in lines {
        let line = line_result.expect("Failed to read line");
        let add_input = line.trim().to_string();
        let add_vec: Vec<&str> = add_input.split_whitespace().collect();

        if add_vec.is_empty() {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0].to_string();
        accounting.push(person);
    }
}

