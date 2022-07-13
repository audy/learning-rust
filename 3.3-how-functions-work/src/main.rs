fn main() {
    let mut x = 6;

    let y = {
        let x = 3;
        // expressions do not include ending semicolons. Adding a semicolon will cause this not to
        // return a value
        x + five()
    };

    another_function(x, y);
    println!("The value of x is {}", x);
}

fn another_function(mut x: i32, y: i32) -> i32 {
    x += 1;
    x
}

fn five() -> i32 {
    plus_one(4)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
