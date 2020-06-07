pub fn run() {
    println!("");
    println!("-------------------------------------------------------------------------------------------------------");
    println!("Hello World from the file hello.rs");
    println!("-------------------------------------------------------------------------------------------------------");
    println!("This is the part about printing");
    println!("");

    println!("{} is {}% fun!", "rust", 100);
    println!("Some numbers {1},{0},{0},{1}", 0, 1);
    println!("{nome},is now existing for {idade} years, that is, during my time, in the year {ano}!", nome="rust", idade=10, ano=2020);
    println!("número 10 em binário:{:b}, em octal seria: {:o}, e em hexa seria: {:x}", 10,10,10);
    println!("number 10 in binary:{:b}, in octal it would be: {:o}, and in hexadecimal it would be: {:x}", 10,10,10);
    println!("eu preciso do super mercado {:?}", ("pão", "miojo", "agua"));
    println!("{:?}", (true, 242, "hello"));
    println!("10 + 10 = {}", 10+10);

    println!("");
    println!("-------------------------------------------------------------------------------------------------------");
}