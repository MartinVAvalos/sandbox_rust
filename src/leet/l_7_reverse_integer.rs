//* Solution 1
fn reverse_integer(x: i32) -> i32 {
    let mut result: Vec<char> = vec![];
    let num_as_string = x.to_string();
    let mut chars = num_as_string.chars();

    if num_as_string.starts_with('-') {
        result.push('-');
        chars.next();
    }

    for ch in chars.rev() {
        result.push(ch);
    }

    result.iter().collect::<String>().parse().unwrap_or(0)
}

//* Solution 2
pub fn reverse(x: i32) -> i32 {
    let mut acc = 0;
    let mut remaining_digits = x;

    while remaining_digits != 0 {
        let pop = remaining_digits % 10;
        remaining_digits /= 10;
        if acc > i32::MAX / 10 {
            return 0;
        }
        if acc < i32::MIN / 10 {
            return 0;
        }
        acc = acc * 10 + pop;
    }
    acc
}

fn main() {
    print!("{}", reverse(120));
}

// Example 1:
// Input: x = 123
// Output: 321

// Example 2:
// Input: x = -123
// Output: -321

// Example 3:
// Input: x = 120
// Output: 21
