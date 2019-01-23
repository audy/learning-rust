fn main() {
    // numbers
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remained = 43 % 5;

    // bools

    let t = true;

    let f: bool = false;

    // chars

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple compounds

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is : {}", y);

    // array compounds
    // stored in the stack
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];

    let second = a[1];

    // runtime, not compiletime, error: a[7]

    /*

    "The compilation didn’t produce any errors, but the program resulted in a runtime error and
    didn’t exit successfully. When you attempt to access an element using indexing, Rust will check
    that the index you’ve specified is less than the array length. If the index is greater than the
    length, Rust will panic."

    TODO: why didn't the compiler notice this?

    */
}
