use std::{cmp::Ordering, collections::HashMap, println};

pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let res = 0;

    let mut map = HashMap::<i32, i32>::new();
    let mut sums : Vec<(i32, i32)> = nums.iter().enumerate().map(|tuple|{

        let x  = tuple.1;
        let digits = x.to_string();
        let mut sum = 0;
        for t in digits.chars().enumerate() {
            let digit = t.1.to_digit(10).unwrap() as i32;
            sum = sum + digit;
        };
        
        return (*x, sum);
    }).collect();

    for (idx, x) in nums.iter().enumerate(){
        
    }

    println!("The sums is {:?}", sums);
    sums.sort_by(|a,b |{
        if a.1 > b.1 {Ordering::Greater} else {
            if a.1 == b.1 && a.0 > b.0 {Ordering::Greater} else {Ordering::Less}
        }
    });
    println!("The sorted sums is {:?}", sums);
    println!("The map is {:?}", map);

    return res;
}

fn main() {
    println!("Hello, world!");

    let test = [1, 1002, 34, 21, 43, 12];
    min_swaps(test.to_vec());
}
