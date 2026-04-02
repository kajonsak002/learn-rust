
// การสร้าง function รูปแบบ fn ชื่อ function (parameter:type) -> i32 {} -> หมายถึงจะ return เป็นอะไรตามด้วย type ตย. -> i32
fn add(x:i32, y:i32) -> i32 {
    x+y
} 

fn main() {
    println!("{}",add(10, 10));
}
