// กล่องเก็บข้อมูล เเละชนิดข้อมูลของเเต่ละตัว
struct Point {
    x: i32,
    y: i32,
}

fn main(){
    let p = Point{x:15,y:16};
    println!("{} {}",p.x,p.y);
}