use std::{cmp::Ordering, collections::HashMap, println};

pub fn min_swaps(nums: &Vec<i32>) -> i32 {

    let mut map = HashMap::<i32, i32>::new();
    let sums : Vec<(i32, i32)> = nums.iter().enumerate().map(|tuple|{

        let x  = tuple.1;
        let digits = x.to_string();
        let mut sum = 0;
        for t in digits.chars().enumerate() {
            let digit = t.1.to_digit(10).unwrap() as i32;
            sum = sum + digit;
        };

        map.insert(*x, tuple.0 as i32);
        
        return (*x, sum);
    }).collect();

    let mut initial_sum = sums;
    let mut sorted_sum = initial_sum.clone();
    sorted_sum.sort_by(|a,b |{
        if a.1 > b.1 {Ordering::Greater} else {
            if a.1 == b.1 && a.0 > b.0 {Ordering::Greater} else {Ordering::Less}
        }
    });

    let mut min_swaps = 0;
    for idx in 0..initial_sum.len()
    {
        let (val, _) = initial_sum[idx];
        let (target_val, _) = sorted_sum[idx];
        if target_val != val {
            min_swaps = min_swaps + 1;
            
            // Old value index is updated with the position of the target
            let target_idx = map[&target_val];
            map.insert(val, target_idx);
            initial_sum[target_idx as usize] = (val, 0);
        }
    }

    return min_swaps;
}

fn main() {

    let tests = [
                                vec![37, 100], 
                                vec![22, 14, 33, 7],
                                vec![1, 1002, 34, 21, 43, 12]
                                ];
    for x in tests.iter().enumerate() {
        let min_swaps = min_swaps(x.1);   
        println!("The min swaps are {}", min_swaps);
    }        
    
}
