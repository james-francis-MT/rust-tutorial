fn main() {
    // mutable variables
    /*
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    */

    /* shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // prints 12
    }
    println!("The value of x is: {x}"); //prints 6
    */

    // chars
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    //tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}"); // prints 6.4

    let five_hundred = tup.0;
    let six_point_4 = tup.1;
    let one = tup.2;

    // Arrays

    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    let first = a[0];
    let second = a[1];

    another_function();
}

fn another_function() {
   println!("Another function."); 
}

// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
