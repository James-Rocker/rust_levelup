// Note, these tests were from the codewars find_short kata. I did not write these
// Kata - https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9


#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng, seq::SliceRandom};
    use rust_kata_exercises::find_short;

    fn do_test(s: &str, expected: u32) {
        let actual = find_short(s);
        assert_eq!(actual, expected, "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        do_test("bitcoin take over the world maybe who knows perhaps", 3);
        do_test("turns out random test cases are easier than writing out basic ones", 3);
        do_test("lets talk about javascript the best language", 3);
        do_test("i want to travel the world writing code one day", 1);
        do_test("Lets all go on holiday somewhere very cold", 2);
        do_test("Let's travel abroad shall we", 2);
        do_test("Steem Lisk Classic Dash Ethereum Dogecoin MadeSafeCoin Ethereum Monero Monero LiteCoin ProofOfWork", 4);
        do_test("21inc Dash LiteCoin Ripple MadeSafeCoin Bitcoin Bitcoin Bitcoin Monero Ripple Bitcoin", 4);
        do_test("Ethereum Factom Monero 21inc Lisk Dogecoin DarkCoin Monero Ethereum Bitcoin ProofOfWork LiteCoin", 4);
        do_test("Lisk BTC DarkCoin Monero", 3);
        do_test("LiteCoin", 8);
        do_test("Waves 21inc LiteCoin Mine Dogecoin Monero LiteCoin", 4);
        do_test("Waves MadeSafeCoin 21inc BTC Steem Lisk Dogecoin Factom ProofOfStake Classic Classic BTC Ethereum Waves Factom Ripple ProofOfStake Classic Steem", 3);
        do_test("Ripple ProofOfWork Mine Lisk Mine Waves Ethereum", 4);
        do_test("DarkCoin", 8);
        do_test("Dogecoin Lisk ProofOfStake Bitcoin LiteCoin Steem DarkCoin Factom Ethereum MadeSafeCoin", 4);
        do_test("Monero Classic Dash Monero Dash Bitcoin Lisk Monero Steem ProofOfWork ProofOfStake LiteCoin Bitcoin Dash", 4);
        do_test("Steem LiteCoin Bitcoin BTC Waves Bitcoin Classic Ethereum", 3);
        do_test("Ripple ProofOfWork MadeSafeCoin Monero Ripple 21inc Waves Monero Dash", 4);
        do_test("DarkCoin BTC Bitcoin Ripple ProofOfStake 21inc MadeSafeCoin Mine Lisk Classic LiteCoin 21inc Waves BTC Steem 21inc Factom Dogecoin Classic", 3);
        do_test("Mine Dash Monero", 4);
        do_test("Lisk Steem Monero Waves Lisk Factom Classic Waves Dogecoin Ripple Steem", 4);
        do_test("Lisk Classic Waves MadeSafeCoin LiteCoin DarkCoin 21inc Ethereum LiteCoin BTC", 3);
        do_test("LiteCoin MadeSafeCoin LiteCoin Dogecoin", 8);
        do_test("Monero Dogecoin ProofOfWork ProofOfStake BTC 21inc Lisk Bitcoin LiteCoin BTC BTC Monero Monero Dogecoin Steem Steem", 3);
        do_test("DarkCoin Factom Steem Steem BTC Classic LiteCoin Lisk Ethereum ProofOfWork Monero Waves Bitcoin Steem MadeSafeCoin", 3);
        do_test("Classic Lisk Bitcoin Mine LiteCoin Factom Ripple BTC 21inc Ethereum LiteCoin Lisk", 3);
        do_test("Monero ProofOfWork Dash LiteCoin Dash Dash ProofOfWork Ethereum Bitcoin DarkCoin", 4);
        do_test("Classic 21inc LiteCoin BTC Lisk DarkCoin 21inc BTC", 3);
        do_test("LiteCoin Dash Dogecoin Ethereum Ethereum Steem Waves Steem 21inc Steem Bitcoin Ethereum MadeSafeCoin Monero Waves 21inc ProofOfWork Dash Steem Lisk", 4);
        do_test("ProofOfWork Waves Mine Ripple DarkCoin Waves Ethereum 21inc Ripple ProofOfStake Waves MadeSafeCoin Lisk ProofOfWork DarkCoin Bitcoin Waves ProofOfStake", 4);
        do_test("Ripple ProofOfStake Dogecoin Mine 21inc BTC LiteCoin Dogecoin MadeSafeCoin Dogecoin Waves Waves MadeSafeCoin DarkCoin Ripple ProofOfWork Lisk ProofOfWork Bitcoin", 3);
        do_test("LiteCoin 21inc Classic Monero Ethereum Factom Dash MadeSafeCoin Ripple ProofOfWork Bitcoin", 4);
        do_test("Monero MadeSafeCoin DarkCoin MadeSafeCoin Lisk Waves BTC DarkCoin", 3);
        do_test("Bitcoin LiteCoin BTC LiteCoin Waves ProofOfWork 21inc", 3);
        do_test("21inc Monero Dash BTC Lisk Ripple Lisk Factom", 3);
        do_test("21inc", 5);
        do_test("Mine ProofOfStake Monero Lisk BTC Steem Steem Monero Classic ProofOfStake Steem", 3);
        do_test("Bitcoin LiteCoin Monero MadeSafeCoin Lisk LiteCoin MadeSafeCoin ProofOfStake Factom Steem DarkCoin", 4);
        do_test("Factom Lisk Classic Ripple Factom DarkCoin LiteCoin", 4);
        do_test("DarkCoin Waves MadeSafeCoin ProofOfWork Dash Mine ProofOfWork", 4);
        do_test("Ripple MadeSafeCoin Dogecoin Ripple Waves Ethereum ProofOfStake 21inc", 5);
        do_test("ProofOfWork Dogecoin", 8);
        do_test("Classic Waves Factom Waves MadeSafeCoin Mine Steem Monero Dogecoin Bitcoin Classic Dogecoin Bitcoin MadeSafeCoin Ripple Mine", 4);
        do_test("Ripple Mine", 4);
        do_test("BTC ProofOfWork MadeSafeCoin Bitcoin Bitcoin Mine Mine Steem Steem", 3);
    }

    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        let all_chars = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!#$%&'*+,-./:;<=>?@^_`~".chars().collect::<Vec<_>>();
        for _ in 0..100 {
            let mut words = Vec::with_capacity(rng.gen_range(1..20));
            let mut expected = 1000;
            for _ in 0..words.capacity() {
                let l = rng.gen_range(1..40);
                expected = expected.min(l);
                words.push((0..l).map(|_| all_chars.choose(&mut rng).unwrap()).collect::<String>());
            }
            do_test(&words.join(" "), expected);
        }
    }
}