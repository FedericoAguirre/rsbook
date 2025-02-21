fn main() {
    scope();
    mut_string();
    // mem_alloc_error();
    mem_auto_release();
    mem_cloning();
    stack_mem_copy();
}

fn scope() {
    let _s = "hello";
}

fn mut_string() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`
}

fn mem_alloc_error() {
    let s1 = String::from("hello");
    // let s2 = s1;
    let s2 = s1.clone();

    println!("{s1}, world!");
}

fn mem_auto_release() {
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");
}

fn mem_cloning(){
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn stack_mem_copy(){
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}