fn main() {
    let x = 6;

    let y = {
        let x = 3;
        // expressions do not include ending semicolons. Adding a semicolon will cause this not to return a
        // value
        x + five()
    };

    another_function(x, y);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    plus_one(4)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
