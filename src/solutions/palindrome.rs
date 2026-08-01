/*This is to check wether a number is palindrome or not
This is the most optimal solution as we are only reversing half of the numbers 
if the number is odd then we drop it by doing the last reverse/10 ==x 
also we are checking the edge cases like negative numbers and numbers
ending with 0 except 0 itself
*/
pub fn is_palindrome(x: i32) -> bool{
    if x <0 || (x % 10 ==0&& x !=0){
        return false;
    }
    let mut x = x; 
    let mut reverse = 0 ; 
    while x > reverse {
        reverse = reverse * 10 + x%10;
        x = x/10;
    }
    if reverse==x ||reverse/10 == x{
        return true;
    }
    false
}