pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let length = x.to_string().len() as u32;
    if length == 1 {
        return true;
    }
    let mut counter: u32 = 0;
    while counter < length / 2 {
        let left = (x / 10_i32.pow(length - counter - 1)) % 10;
        let right = (x / 10_i32.pow(counter)) % 10;
        if left != right {
            return false;
        }
        counter += 1;
    }
    true
}

fn main() {
    println!("{}", is_palindrome(13221));
}
