use std::collections::HashMap;

//here we will do both the brute force as well as optimzed answers 
//Brute force Method
pub fn two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
    let n = nums.len();
    for i in 0..n {
        for j in (i+1)..n{
            if nums[i] + nums[j] == target{
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

/* Before optimizing this code we will see what is the things that we need to optimize
1. The time complexity is O(n^2) which we will clearly reduce 
To do this we will trade the time  with the space , So we will use the concept of the Hashmap
*/

pub fn two_sum_optimized(nums:Vec<i32>,target:i32)->Vec<i32>{
    let mut map = HashMap::with_capacity(nums.len());
    for(i, &num) in nums.iter().enumerate(){
        let complement = target-num;

         if let Some(&prev_index) =map.get(&complement){
            return vec![prev_index as i32, i as i32];
         } 
        map.insert(num, i);
    }
    vec![]
}
