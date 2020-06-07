pub fn run() {
    println!("Hello World from the file types.rs");
    println!("-------------------------------------------------------------------------------------------------------");
    println!("");

    println!("printing a variable that was explicitly assigned to 64 bits of space to store the variable x:");
    let x: i64 = 999999999999;
    println!("x = {}", x);
    println!("");

    println!("Finding the max size of the data type:");
    println!("Max of i32 is: {}", std::i32::MAX);
    println!("Max of i32 is: {}", std::i64::MAX);
    println!("");

    println!("Testing boolean types of variables:");
    let is_true: bool = true;
    let is_bigger: bool = 100 > 1;
    let is_bigger_fail: bool = 100 < 1;
    println!("{:?}", (x, is_true, is_bigger, is_bigger_fail));
    println!("");

    println!("Testing other types of variables:");
    let a1 = 'a';
    let face = '\u{1f600}';
    println!("{:?}", (a1, face));

    println!("");
    println!("-------------------------------------------------------------------------------------------------------");
}