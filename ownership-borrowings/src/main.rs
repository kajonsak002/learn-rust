fn main() {
    // =========================
    // 🔥 SECTION 1: MOVE (ownership ย้าย)
    // =========================
    let s1 = String::from("Hello MOVE");

    // ส่งเข้า function → ownership ย้ายไป
    take_ownership(s1);

    // ❌ ใช้ไม่ได้แล้ว (จะ error ถ้า uncomment)
    // println!("{}", s1);


    // =========================
    // 🔥 SECTION 2: BORROW (ยืมค่า)
    // =========================
    let s2 = String::from("Hello BORROW");

    // ส่ง reference → ไม่ย้าย ownership
    borrow_value(&s2);

    // ✅ ยังใช้ได้
    println!("after borrow: {}", s2);


    // =========================
    // 🔥 SECTION 3: &str (ไม่เกิด move)
    // =========================
    let s3 = "Hello STR";

    // ส่งเข้า function → copy reference เฉย ๆ
    use_str(s3);

    // ✅ ยังใช้ได้
    println!("after &str: {}", s3);
}


// =========================
// FUNCTION: MOVE
// =========================
fn take_ownership(s: String) {
    // s เป็น owner ใหม่
    println!("inside MOVE: {}", s);

    // จบ function → s ถูก drop
}


// =========================
// FUNCTION: BORROW
// =========================
fn borrow_value(s: &String) {
    // ยืมมาใช้ → ไม่เป็น owner
    println!("inside BORROW: {}", s);

    // ไม่ drop เพราะไม่ได้เป็นเจ้าของ
}


// =========================
// FUNCTION: &str
// =========================
fn use_str(s: &str) {
    println!("inside &str: {}", s);
}