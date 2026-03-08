// functions4.rs
//
// 这家店正在促销：如果价格是偶数，可以减免10个Rust币；如果是奇数，则减免3个Rust币。
//（别担心函数体本身，我们目前只关心函数签名。无论如何，这都是提前了解后续练习的好方法！）
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
