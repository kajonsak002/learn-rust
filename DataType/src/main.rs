use std::any::type_name; //lib นอกไว้สำหรับตรวจสอบ DataType

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {

    // การประกาศตัวแปรจะใช้ let ตามด้วย ชื่อ เเละชนิดของตัวแปร
    let name = "kajonsak";

    // การแสดงผลจะใช้คำสั่ง println("{}", ตัวแปร)
    println!("Hello {}",name);

    let number = 10;
    println!("{}",number);

    //การประกาศตัวแปรพร้อมกำหนด Type ตัวอย่าง :i32 หมายถึง int32 bit
    let x:i32 = 100;
    let z:bool = true;
    let c:char = 'a';
    println!("x = {} z = {} c = {}",x,z,c);

    //การตรวจสอบ DataType
    println!(" type x = {}",type_of(&x))

}
