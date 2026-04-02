// การสร้าง macros rule จะเเบ่งเป็น 2 แบบ Macros Rule and Procedural Macro
macro_rules! say_hello {
    () => (println!("Hello!"));
}
fn main() {
    say_hello!();
}
