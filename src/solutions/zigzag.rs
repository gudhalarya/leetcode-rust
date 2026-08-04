//This is the solution of the problem zigzag 
pub fn convert(s: String, num_rows: i32) -> String{
    if num_rows == 1 || num_rows as usize >= s.len(){
        return  s ;
    }
    let mut current_row = 0 ; 
    let mut going_down = false;
    let mut rows  = vec![String::new();num_rows as usize];
    for c in s.chars(){
        rows[current_row].push(c);
        if current_row == 0 || current_row == num_rows as usize -1{
            going_down = !going_down;
        }
        if going_down{
            current_row += 1; 
        }
        else {
            current_row -=1;
        }
    }
    rows.concat()
}