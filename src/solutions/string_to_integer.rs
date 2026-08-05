impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.trim_start().chars().peekable();

        let mut sign = 1;

        if let Some(&ch) = chars.peek() {
            if ch == '-' {
                sign = -1;
                chars.next();
            } else if ch == '+' {
                chars.next();
            }
        }

        let mut ans: i32 = 0;

        while let Some(&ch) = chars.peek() {
            if !ch.is_ascii_digit() {
                break;
            }

            let digit = ch.to_digit(10).unwrap() as i32;

            if sign == 1 {
                if ans > i32::MAX / 10
                    || (ans == i32::MAX / 10 && digit > 7)
                {
                    return i32::MAX;
                }

                ans = ans * 10 + digit;
            } else {
                if ans < i32::MIN / 10
                    || (ans == i32::MIN / 10 && digit > 8)
                {
                    return i32::MIN;
                }

                ans = ans * 10 - digit;
            }

            chars.next();
        }

        ans
    }
}