fn main() {
    scope();
    mut_string();
    // mem_alloc_error();
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
