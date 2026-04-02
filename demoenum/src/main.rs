// enum = ใช้สร้างชนิดข้อมูลที่มี “ตัวเลือกหลายแบบ” (เลือกได้แค่ 1 ค่า)
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    // สร้างตัวแปร c, g, b จาก enum Color
    let c: Color = Color::Red;     // c เก็บค่า Red
    let g: Color = Color::Green;   // g เก็บค่า Green
    let b: Color = Color::Blue;    // b เก็บค่า Blue

    // match = ใช้เช็คค่า (เหมือน switch-case แต่บังคับต้องครบทุกกรณี)
    match c {
        Color::Red => println!("RED"),     // ถ้า c เป็น Red
        Color::Green => println!("Green"), // ถ้า c เป็น Green
        Color::Blue => println!("Blue"),   // ถ้า c เป็น Blue
    }

    // เช็คค่า g
    match g {
        Color::Red => println!("RED"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    // เช็คค่า b
    match b {
        Color::Red => println!("RED"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}