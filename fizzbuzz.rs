fn div_by_three(num: int) -> bool {
    num % 3 == 0
}

fn div_by_five(num: int) -> bool {
    num % 5 == 0
}

fn div_by_fifteen(num: int) -> bool {
    num % 15 == 0
}

fn main() {
    for num in range(1i, 101) {
        println!("{:s}",
            if div_by_fifteen(num) { "FizzBuzz".to_string() }
            else if div_by_five(num) { "Buzz".to_string() }
            else if div_by_three(num) { "Fizz".to_string() }
            else { num.to_string() }
        );
    }
}

#[test]
fn test_div_by_three() {
    assert!(!div_by_three(1))
    assert!(div_by_three(3))
}

#[test]
fn test_div_by_five() {
    assert!(!div_by_five(1))
    assert!(div_by_five(5))
}

#[test]
fn test_div_by_fifteen() {
    assert!(!div_by_fifteen(1))
    assert!(div_by_fifteen(15))
}
