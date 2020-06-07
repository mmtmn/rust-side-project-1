pub fn run() {
    println!("Hello World from the file vars.rs");
    println!("-------------------------------------------------------------------------------------------------------");
    let section = "This section is for variables";
    println!("{}", section);
    println!("");

    let mut mutable_var = 50;
    println!("mutable var is {}", mutable_var);
    mutable_var = 51;
    println!("mutable var is now {}", mutable_var);
    println!("");

    println!("printing a varible with explicitly telling it to have 32 bits of space of storage:");
    const ID: i32 = 001;
    println!("ID: {}", ID);

    println!("");
    println!("-------------------------------------------------------------------------------------------------------");
}