use std::io;

fn main() {
    const CONSTANT_TEST: u32 = 60;
    println!("hello world!");
    println!("{}", test());
    let tup = test_tuple();
    println!("{}", tup.2);
    println!("{}", readFromTerminal());
}

fn readFromTerminal() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line"); 
    //the reference is required to make it point to this correctly for modifying
    return input;
}

fn test() -> i32 { //-> i32 for return type
    let mut x = 4; //use mut to make mutable
    let y = 3;
    let y = 4; //redeclare to change value
    let z: i32 = 2;
    x += y+z;
    pitfall();
    test_bool();
    test_arr();
    return x;
}

fn test_bool() -> bool {
    return true;
}
fn test_tuple() -> (i32, bool, char) {
    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.1);
    return tup;
}

fn test_arr() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; //need to manually initialize the values..
}

fn pitfall() {
    let x = 10;
    let mut y = 5;

    {
        let x = 2; //only changed within the scope
        y = 4;
    }

    println!("{}", x); //prints 10 as the other variable is outside scope
    println!("{}", y); //prints 4 as the mutable is changed
    let x = "can change the type by using let again";
    println!("{}", x);
}

fn test_condition() {
    if(5 >= 6) {
        println!("{}", "not true");
    }
}