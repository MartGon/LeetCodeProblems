use std::{collections::HashMap, println};

pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let res = 0;
    println!("Hello from min_swaps: {:?}", nums);

    let mut map = HashMap::<i32, i32>::new();
    let sums : Vec<i32> = nums.iter().enumerate().map(|tuple|{

        println!("Hello from enumerate");
        let x  = tuple.1;
        let digits = x.to_string();
        let mut sum = 0;
        for t in digits.chars().enumerate() {
            let digit = t.1.to_digit(10).unwrap() as i32;
            sum = sum + digit;
        };
        println!("The sum is {}", sum);
        map.insert(*x, sum);

        return sum;
    }).collect();

    println!("The result is {:?}", map);

    return res;
}

fn main() {
    println!("Hello, world!");

    let test = [1, 12, 34];
    min_swaps(test.to_vec());
}
