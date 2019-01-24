fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // error because is now owned by takes_ownership (s is not Copy)
    // println!("s: {}", s);

    let s = String::from("hello");
    let s = returns_ownership(s);

    // valid because we re-assigned s
    println!("s: {}", s);

    // no error because x is on the stack and just gets copied (x is Copy)
    let x = 5;

    makes_copy(x);

    // x was copied so is still valid here
    println!("x: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
} // drop is called on some_string. some_string is no valid in the parent scope

fn returns_ownership(some_string: String) -> String {
    println!("some_string: {}", some_string);

    some_string
} // some_string got returned (ownership transferred back to the parent scope)

fn makes_copy(some_int: i32) {
    println!("some_int: {}", some_int);
} // some_int gets dropped but it's a copy so x is still valid in the parent scope
