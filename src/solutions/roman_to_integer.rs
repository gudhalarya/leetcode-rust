pub fn roman_to_int(s: String) -> i32 {
        fn value(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        let chars: Vec<char> = s.chars().collect();
        let mut ans = 0;

        for i in 0..chars.len() {
            let curr = value(chars[i]);

            if i + 1 < chars.len() {
                let next = value(chars[i + 1]);

                if curr < next {
                    ans -= curr;
                } else {
                    ans += curr;
                }
            } else {
                ans += curr;
            }
        }

        ans
    }
