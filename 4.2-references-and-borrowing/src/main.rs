fn main() {
    let s1 = String::from("hello");

    // &s1 means that calculate_length is borrowing s1 but does not own it
    let len = calculate_length(&s1);

    println!("Length is {}", len);

    let mut s2 = String::from("world");

    // you cannot mutate a borrowed value unless you specify that mutation is allowed
    let len2 = mutate_and_calculate_length(&mut s2);

    // returned and mutated values will match because let the function borrow its memory in the
    // heap (the reference was borrowed)
    println!(
        "Mutated Length is {}. Original length is {} because we mutated s1",
        len2,
        s2.len()
    );

    // you can create multiple mutable borrows
    let r1 = &mut s2;
    let r2 = &mut s2;

    // but you can't use them.
    // this prevents data races
    // println!("r1 = {}; r2 = {}", r1, r2);

    //
    // creating immutable references will not allow future mutable references to be created
    //
    let mut s = String::from("hello");

    let r1 = &s; // okay
    let r2 = &s; // okay

    let r3 = &mut s;

    // can't use mutable references together with immutable ones
    // println!("{}, {}, and {}", r1, r2, r3);
}

// the borrowing function has to specify that it takes a borrowed value
fn calculate_length(s: &String) -> usize {
    // won't work because we need to specify mut in a couple of places
    // s.push_str("foo");
    s.len()
}

// the mutating function also has to specify that its input must be mutable
fn mutate_and_calculate_length(s: &mut String) -> usize {
    s.push_str("foo");
    s.len()
}

// this will pass ownership of s to the calling function but since s will be dropped when this
// function goes out of scope it could return a dangling reference which is bad (because we don't
// know what this reference will point to in the future)
//
// interestingly, this function won't even compile because the compiler detects this problem early
// on
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
