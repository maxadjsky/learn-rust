#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    //下面两个是结构体方法
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self,other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    //下面是结构体关联函数 -- 不以self作为参数
    //
    fn square(size: u32) -> Rectangle {
        Rectangle {length: size,width:size}
    }
}

fn main() {
    let rect1 = Rectangle { length:50,width:30};
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("The area of rectangle is {} square piexls.",rect1.area());

    println!("Can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}",rect1.can_hold(&rect3));


    //create a square
    let square = Rectangle::square(30);
    println!("{}",rect1.can_hold(&square));
}
