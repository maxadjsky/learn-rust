fn main() {

//slice 允许引用集合中的一段连续的元素序列

    let s = String::from("hello world!");
    let word = first_word(&s);

    println!("{}",word);

}

fn first_word(s: &String)->&str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
