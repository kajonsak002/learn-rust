fn main() {

    // ประกาศตัวแปร x แบบ immutable (ค่าเปลี่ยนไม่ได้)
    let x = 5;

    // การเช็ค condition สามารถใช้ if ตามด้วย condition ได้เลย if condition { //logic }
    if x > 0{
        println!(" x is positive");
    }else {
        println!(" x is negative");
    }

    let y = ["apple","banana" , "mango"];

    // for loop สามารถใช้ for i in y.iter() เพื่อลูปแต่ละชิ้นออกมา
    for i in y.iter(){
        println!("{}",i)
    }

    // การประกาศตัวแปรแบบ Mutable(เปลี่ยนเเปลงค่าได้)
    let mut z = 15;

    while z < 50 {
        println!(" z = {} ",z);
        z+=15;
    }

    // เหมือน console.log() debug data {:?} แสดงโครงสร้าง (Debug)
    println!("user = {:?}", y);

}
