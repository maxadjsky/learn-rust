
fn main() {
    let length1 = 50;
    let width1 = 30;

    println!("The area of the rectangle is {} square pixels.",
             area(length1,width1));
    
    let rect1 = (50,30);

    println!("The area of the rectangle is {} square pixels.",
             area_with_tuple(rect1));

    let rect2 = Rectangle {length:50,width:30};
    
    println!("The area of the rectangle is {} square pixels.",
             area_with_struct(&rect2));

    println!("rect2 is {:?}",rect2);
}
//  下面的信息可以用pringln！直接打印出结构体信息
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}


fn area(length: u32,width: u32) -> u32{
    length * width
}

fn area_with_tuple(dimensions:(u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
