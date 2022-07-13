struct User {
    active: bool, // implements Copy trait
    username: String,
    email: String,
    sign_in_count: u64, // implements Copy trait
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someone"));

    let user2 = User { ..user1 };

    user1.email = String::from("anotheremail@example.com");

    println!("{}", user2.email);
    println!("{}", user1.email);

    // valid b/c sign_in_count implements Copy
    println!("{}", user1.sign_in_count);

    // invalid b/c username does not implement Copy
    // println!("{}", user1.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
