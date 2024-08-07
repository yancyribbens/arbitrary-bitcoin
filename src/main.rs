use bitcoin_units::Amount;
use proptest::prelude::*;

fn main() {
    let _arb = Amount::arbitrary();
}
