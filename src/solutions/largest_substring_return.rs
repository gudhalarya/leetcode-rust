/*This is where we will find the largest_plaindorme substring in the string 
1. We cant use the hashset here as it does not preserve the insert order so we will discard it 
*/
impl Solution{
pub fn longest_palindrome(s: String) -> String{
    let sq :Vec<char> = s.chars().collect();
    let n = sq.len();
    if n == 0{return String::new()};
    let mut start = 0 ; 
    let mut max_len = 0; 

    for i in 0..n{
        for j in i..n{
            if Self::is_palindorme(&sq,i,j){
                let len = j-i+1;
                if len>max_len{
                    max_len=len;
                    start = i;
                }
            }
        }
    }
    return sq[start..start+max_len].iter().collect();
}

pub fn is_palindrome(v:Vec<char>,mut left:usize, mut right:usize)->bool{
    while left<right{
        if v[&left] != v[right]{
            return false;
        }
        left +=1;
        right = right - 1;
    }
    true
}
}