pub fn my_atoi(s: String) -> i32 {
    let mut bytes = s.trim_start().bytes().peekable();
    let is_neg = matches!(bytes.peek(), Some(&b'-'));
    if matches!(bytes.peek(), Some(&b'-') | Some(&b'+')) {
        bytes.next();
    }

    let mut acc: i32 = 0;
    let mut last_digit_limit = if is_neg { 8 } else { 7 };

    for byte in bytes {
        if !byte.is_ascii_digit() {
            break;
        }
        let digit = (byte - b'0') as i32;
        if acc < i32::MIN / 10 || (acc == i32::MIN / 10 && digit > last_digit_limit) {
            return if is_neg { i32::MIN } else { i32::MAX };
        }

        acc = acc * 10 - digit
    }

    if is_neg {
        acc
    } else {
        -acc
    }
}

pub fn main() {
    my_atoi("-100".to_string());
}
