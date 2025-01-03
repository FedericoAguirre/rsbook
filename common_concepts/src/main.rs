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
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    // Accesing tuple elements using . and index
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");

    // Array type

    let _a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // explicit type and size
    let first = a[0];
    let second = a[1];
    println!("first: {first}, second: {second}");

    // Functions
    // Another function call
    another_function();

    // Function with parameters
    another_function_with_param(5);
    print_labeled_measurement(5, 'h');

    // Statements and expressions
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Control flow
    // if expressions

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if else expressions
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Repeating Code with loop

    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loops with break, continue and labels

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loop

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loop

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // looping through elements

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // Looping in reverse

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // Farenheit convertion

    let farennheit: f64 = 60.0;
    let celsius: f64 = farenheit_to_celsius(farennheit);
    println!("{farennheit} F is equal to {celsius} C");

    // Fibonacci

    let limit: u128 = 10;

    for n in 0..limit {
        let f = fibonacci(n);
        println!("Fibonacci of {n} is {f}.");
    }

    // 12 days of Christmas

    twelve_days();
}

// Functions

// Function definition

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn farenheit_to_celsius(farennheit: f64) -> f64 {
    (farennheit - 32.0f64) * 5.0f64 / 9.0f64
}

fn fibonacci(n: u128) -> u128 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn twelve_days() {
    let days: [String; 12] = [
        String::from("first"),
        String::from("second"),
        String::from("third"),
        String::from("fourth"),
        String::from("fifth"),
        String::from("sixth"),
        String::from("seventh"),
        String::from("eighth"),
        String::from("ninth"),
        String::from("tenth"),
        String::from("eleventh"),
        String::from("twelfth"),
    ];

    let phrases: [String; 12] = [
        String::from("A partridge in a pear tree"),
        String::from("Two turtle doves"),
        String::from("Three French hens"),
        String::from("Four calling birds"),
        String::from("Five gold rings"),
        String::from("Six geese a-laying"),
        String::from("Seven swans a-swimming"),
        String::from("Eight maids a-milking"),
        String::from("Nine ladies dancing"),
        String::from("Ten lords a-leaping"),
        String::from("Eleven pipers piping"),
        String::from("Twelve drummers drumming"),
    ];

    let mut actual = 0;

    for day in days {
        println!("On the {day} day of Christmas my true love sent to me");
        let mut index = 0;

        while index <= actual {
            let phrase = &phrases[actual - index];
            if index == actual {
                println!("{phrase}\n");
            } else {
                println!("{phrase}, and");
            }
            index += 1;
        }

        actual += 1;
    }
}
