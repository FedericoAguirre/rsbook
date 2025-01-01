fn main() {


    // floating point variables

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x: {x}, y: {y}");


    // Operators

    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("quotient: {quotient}, truncated: {truncated}");


    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");


    // Boolean type
    
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("true: {t}, false: {f}");


    // Character type
    
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");


    // Compound types
    
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {tup:?}");
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // Accesing tuple elements using . and index
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");


    // Array type

    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // explicit type and size
    let first = a[0];
    let second = a[1];
    println!("first: {first}, second: {second}");

}