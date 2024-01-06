// Note, these tests were from the codewars find the divisors kata. I did not write these
// Kata - https://www.codewars.com/kata/544aed4c4a30184e960010f4
use rust_kata_exercises::divisors;

#[test]
fn simple_examples() {
    assert_eq!(divisors(15), Ok(vec![3,5]));
    assert_eq!(divisors(253), Ok(vec![11,23]));
    assert_eq!(divisors(24), Ok(vec![2,3,4,6,8,12]));

    assert_eq!(divisors(13), Err("13 is prime".to_string()));
    assert_eq!(divisors(3), Err("3 is prime".to_string()));
    assert_eq!(divisors(29), Err("29 is prime".to_string()));
}