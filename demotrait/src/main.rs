// struct = ใช้เก็บข้อมูลของสิ่งหนึ่ง (entity)

// Person มี field name
struct Person {
    name: String, // ต้องมี , ปิดท้าย
}

// Cat มี field name เหมือนกัน
struct Cat {
    name: String,
}

// trait = กำหนด “พฤติกรรมร่วม” (เหมือน interface ใน TS)
trait Eat {
    // ฟังก์ชันที่ทุกคนที่ implement trait นี้ต้องมี
    fn eat_dinner(&self);
}

// impl = การเอา trait ไปใช้กับ struct

// บอกว่า Person มีพฤติกรรม Eat
impl Eat for Person {
    fn eat_dinner(&self) {
        // self = ตัว object ปัจจุบัน
        println!("I eat from plate");
    }
}

// บอกว่า Cat ก็มีพฤติกรรม Eat เหมือนกัน
impl Eat for Cat {
    fn eat_dinner(&self) {
        println!("I eat from bowl");
    }
}

fn main() {
    // สร้าง object Person
    let person = Person {
        name: String::from("F"),
    };

    // เรียก method จาก trait
    // Rust จะรู้ว่า Person implement Eat
    person.eat_dinner();

    // สร้าง object Cat
    let cat = Cat {
        name: String::from("Pong"),
    };

    // Cat ก็ใช้ method เดียวกันได้
    cat.eat_dinner();
}