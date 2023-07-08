// Exercise 1
// Make it compile
fn exercise1() {
    let x = String::from("hello, world");
    let y = x.clone(); // cloning `x` to `y`
    let z = x.clone(); // cloning `x` to `z`
    println!("{}, {}, {}", x, y, z);
}

// Exercise 2
// Make it compile
// Don't modify code in exercise2 function!
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1.clone()); // cloning `s1`

    println!("{}", s2);
}
// Only modify the code below!
fn take_ownership(s: String) -> String { // returning the string
    println!("{}", s);
    s
}

// Exercise 3
// Make it compile
// Dont care about logic
fn exercise3() {
    let values: Vec<f64> = vec![
        // values
    ];
    let values_number = values.len();
    let mut additions: Vec<usize> = vec![0];
    println!("{:?}", values_number);

    while additions.len() > 0 {
        let mut addition: f64 = 0.0;

        for element_index in &additions { // using reference to avoid ownership issue
            addition += values[*element_index];
        }

        additions.pop(); // removing elements to prevent infinite loop
    }
}

// Exercise 4
// Make it compile
fn exercise4(value: u32) -> String { // changed the return type to String
    let str_value = value.to_string();
    str_value
}

// Exercise 5
// Make it compile
use std::collections::HashMap;

fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);
    let key = 3;
    let res = match my_map.get(&key) {
        Some(child) => child.clone(), // cloning the child
        None => {
            let value = "3.0".to_string();
            my_map.insert(key, value.clone()); // clone the value to be inserted
            value
        }
    };
    println!("{}", res);
}

// Exercise 6
// Make it compile

use std::io;
use std::io::BufRead;  // Add this import

fn exercise6() {
    let mut prev_key = String::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {  // Remove .lock()
        let s = line.unwrap();
        let data: Vec<&str> = s.split("\t").collect();
        if prev_key.is_empty() {
            prev_key = data[0].to_string();
        }
    }
}


// Exercise 7
// Make it compile
fn exercise7() {
    let mut v: Vec<String> = Vec::new(); // Vector of String
    {
        let chars = [b'x', b'y', b'z'];
        let s: String = std::str::from_utf8(&chars).unwrap().to_owned();
        v.push(s);
    }
    println!("{:?}", v);
}

// Exercise 8
// Make it compile
fn exercise8() {
    let mut accounting = vec!["Alice".to_string(), "Ben".to_string()];
    
    loop {
        let mut add_input = String::new();
        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<&str> = add_input.trim().split_whitespace().collect();
        if add_vec.is_empty() {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0].to_string(); // convert to String
        accounting.push(person);
        // Some condition to break the loop or it will run forever
    }
}
