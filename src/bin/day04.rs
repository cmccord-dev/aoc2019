
fn isValid(num_p: i32) -> bool {
    let mut last_digit = 11;
    let mut has_double = false;
    let mut num = num_p;
    while num > 0 {
        let digit = num % 10;
        has_double |= last_digit == digit;
        if digit > last_digit {
            return false;
        }
        last_digit = digit;
        num = num / 10;
    }
    has_double
}

fn isValid_part2(num_p: i32) -> bool {
    let mut last_digit = 11;
    let mut has_double = false;
    let mut num = num_p;
    let mut digit_count = 0;
    while num > 0 {
        let digit = num % 10;
        //dbg!((digit, last_digit, digit_count, has_double));
        if last_digit == digit {
            digit_count += 1;
        } else {
            has_double |= digit_count == 1;
            digit_count = 0;
        }
        if digit > last_digit {
            return false;
        }
        last_digit = digit;
        num = num / 10;
    }
    has_double || digit_count == 1
}
fn main() {
    let min: i32 = 108457;
    let max: i32 = 562041;
    dbg!(isValid_part2(112233));
    dbg!(isValid_part2(123444));
    dbg!(isValid_part2(111122));
    dbg!((min..max)
        
        .map(|i: i32| if isValid_part2(i) { 1 } else { 0 })
        .sum::<i32>());
}
