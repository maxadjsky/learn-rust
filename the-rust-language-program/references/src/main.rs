fn main() {

    let s1 = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.",s1,len);

    let mut ss = String::from("Hello");

    change(&mut ss);
}

fn calculate_length(s:&String) -> usize {
    s.len() //参数引用不用返回值，因为本身就没有所有权
}

//可变引用
//可变引用的限制：在特定作用域中的特定数据有且仅有一个可变引用
fn change(some_string: &mut String){
    some_string.push_str(", world");
}
