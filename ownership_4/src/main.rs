fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
    //mut_ref_error();
    mut_ref();
    // single_mut_ref_error();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//fn mut_ref_error() {
//    let mut s = String::from("hello");
//    change(&s);
//}

//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

fn mut_ref() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("s: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn single_mut_ref_error() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);

// }