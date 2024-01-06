// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// Note, these tests were from the codewars flick switch kata. I did not write these
// Kata - https://www.codewars.com/kata/64fbfe2618692c2018ebbddb


#[cfg(test)]
mod tests {
    use rust_kata_exercises::flick_switch;

    use std::borrow::Borrow;
    use std::borrow::Cow;

    use rand::thread_rng;
    use rand::Rng;
    use rand::rngs::ThreadRng;
    use rand::seq::SliceRandom;

    fn test_flick<'a, S: Borrow<[&'a str]>, E: Borrow<[bool]>>(strings: S, expected: E) {
        let strings: &[&'a str] = strings.borrow();
        let expected: &[bool] = expected.borrow();
        assert_eq!(flick_switch(strings), expected);
    }

    #[test]
    fn fixed_tests() {
        test_flick(["codewars", "flick", "code", "wars"], [true, false, false, false]);
        test_flick(["flick", "11037", "3.14", "53"], [false, false, false, false]);
        test_flick(["false", "false", "flick", "sheep", "flick"], [true, true, false, false, true]);
        test_flick(["bicycle"], [true]);
        test_flick(["john, smith, susan", "flick"], [true, false]);
        test_flick(["flick", "flick", "flick", "flick", "flick"], [false, true, false, true, false]);
        test_flick([], []);
    }

    #[test]
    fn random_tests() {
        let mut rng = thread_rng();

        fn generate_random_value<'a, 'b>(rng: &'b mut ThreadRng) -> Cow<'a, str> {
            let value_type: &'static str = ["string", "flick", "flick"].choose(rng).unwrap();

            if value_type == "string" {
                let length = rng.gen_range(1..10);

                const ASCII_LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
                let random_string: String = (0..length)
                    .map(|_| *ASCII_LETTERS.choose(rng).unwrap() as char)
                    .collect();
                Cow::Owned(random_string)
            } else {
                Cow::Borrowed("flick")
            }
        }

        fn solve(list: &[&str]) -> Vec<bool> {
            list.iter().copied()
                .scan(true, |is_on: &mut bool, item: &str| {
                    *is_on ^= item == "flick";
                    Some(*is_on)
                })
                .collect()
        }

        for _ in 0..100 {
            let values: Vec<Cow<str>> = (0..rng.gen_range(0..20))
                .map(|_| generate_random_value(&mut rng))
                .collect();
            let list: Vec<&str> = values.iter().map(|item| item.borrow()).collect();
            let solution = solve(list.as_slice());
            let result = flick_switch(list.as_slice());
            assert_eq!(solution, result, "It should work for random inputs too");
        }
    }
}