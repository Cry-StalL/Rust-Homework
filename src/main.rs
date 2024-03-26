mod lib;

fn main() {
    println!("{}", lib::acronym("As Soon As Possible"));
    println!("{}", lib::acronym("By The Way"));
    println!("{}", lib::acronym("Liquid-crystal display")); // 大小写混合
    println!("{}", lib::acronym("Thank George It's Friday!"));

    println!("{}", lib::acronym("As   Soon  As    Possible")); // 连续多个空格
    println!("The acronym is: {}", lib::acronym("")); //空字符串
    println!("{}", lib::acronym("    ")); //只有空格
    println!("{}", lib::acronym("As, soon! As Possible.")); //含其他标点符号
    println!("{}", lib::acronym("Don't Panic!")); //含其他标点符号

    println!("{}", lib::acronym("Room 101")); //含数字
    println!("{}", lib::acronym("Room p01")); //数字与字母混合
}
