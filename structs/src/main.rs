struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u32, u32, u32);
struct Point(u32, u32, u32);

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("someusername123"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color is {} {} {}", black.0, black.1, black.2);
    println!("origin is {} {} {}", origin.0, origin.1, origin.2);

    println!("user2 is active :{}", user2.active);
    println!("user2's username is: {}", user2.username);
    println!("user2's email is: {}", user2.email);
    println!("user2 has signed in {} times", user2.sign_in_count);

    println!("User1 is active: {}", user1.active);
    println!("User1's username is: {}", user1.username);
    println!("User1's email is: {}", user1.email);
    println!("User1 has signed in {} times", user1.sign_in_count);
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
