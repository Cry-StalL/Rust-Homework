// 该函数接受一个字符串，返回一个字符串，返回的字符串是输入字符串的首字母缩略词
// 例如：
// 不含数字时
// As Soon As Possible -> ASAP
// Liquid-crystal display -> LCD
// 含数字时
// Room 101 -> R101

pub fn acronym(input: &str) -> String {
    let words: Vec<&str> = input.split([' ', '-']).collect();

    let mut acronym_str = String::new();

    for word in words {
        let contains_number = word.chars().any(|c| c.is_digit(10));

        if contains_number{
            acronym_str.push_str(&word.to_uppercase()); 
        }else{
            if let Some(c) = word.chars().next(){
                acronym_str.push(c.to_ascii_uppercase());
            }
        }
    }
    return acronym_str;
}