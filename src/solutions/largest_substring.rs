/*This is the 3rd question here to find the largest substring here 
The new thing here is the hashset we will use it now 
1. Unique thing about this is that the hashset is better than a vec[] in any ways such as the lookup time is O(1) whereas in vec[] is O(n)
2. No duplicates are allowded*/

use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32{
    let chars :Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut max = 0; 
    for i in 0..n{
        let mut set = HashSet::new();
        for j in i..n{
            if !set.insert(chars[j]){
                break;
            }
            let current = j-i+1;
            if current>max{
                max=current;
            }
        }
    }
    max as i32
}

//The biggest pain here is this hashset that is being created each time this loop will run hence memory lost each time ---------So just make something athat the hashset is created only once 
//The savior is HashSet + sliding window algo 
pub fn length_of_longest_substring_optimized(s: String) -> i32{
    let seq :Vec<char>= s.chars().collect();
    let n = seq.len();
    let mut  set = HashSet::new();
    let mut  left = 0;
    let mut max_len = 0;

    for right in 0..n{
        while set.contains(&seq[right]){
            set.remove(&seq[left]);
          left += 1;
        }
        set.insert(&seq[right]);
        max_len = max_len.max(right-left+1);
    }
    max_len as i32
}