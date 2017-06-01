#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
//内嵌多种数据类型的枚举
#[derive(Debug)]
enum Message {
    Quit, //没有关联任何数据
    Move { x: i32,y:i32},//包含一个匿名结构体
    Write(String),//包含单独一个String
    ChangeColor(i32,i32,i32),//包含三个i32
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}",home);
    println!("{:?}",loopback);
}
