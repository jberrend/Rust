fn main() {
    let x: i32 = 123;             // immutable by default

    let a = [1, 2, 3]; // a: [i32; 3]
    let b = [0; 20]    // b: [i32; 20]

    let c: (i32, &str) = (1, "Hello");  // tuple

    println!("The value of x is: {}", x);
    print_number(x);
    add_one(2);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1                       // No semicolon - expression not statement
}

fn diverges() -> ! {
    panic!("This function never returns");
}

fn if_five(x: i32) {
    if x == 5 {
        println!("x equals 5!");
    } else {
        println!("x doesn't equal five...");
    }
}
