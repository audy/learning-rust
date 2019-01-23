fn main() {
    let condition = false;

    let number = if condition { three() } else { five() };

    println!("number is {}", number);

    let a = [1, 2, 3, 4, 5];

    let mut index = 0;

    while index < 5 {
        println!("(while) value is {}", a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("(iter) value is {}", element);
    }

    for value in (1..5).rev() {
        println!("(range) value is {}", value);
    }
}

fn three() -> i32 {
    3
}

// compiler type error thanks to type declarations
// fn five() -> char {
//     '5'
// }

fn five() -> i32 {
    5
}
