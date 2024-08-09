use bitcoin_units::Amount;
use proptest::prelude::*;

fn main() {
    let _arb = Amount::arbitrary();
}

proptest! {
    #[test]
    fn prop_test(amt in any::<Amount>()) {
        assert_eq!(amt, amt);
    }
}
