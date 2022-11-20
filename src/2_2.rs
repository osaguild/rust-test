/*
 * call fn and run test
 */
use rust_test::lib::gcd;

fn main() {
    let v = gcd(15, 12);
    println!("{}", v);
}
