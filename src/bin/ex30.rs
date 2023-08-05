
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {

    let user1: User = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("username: {}", user1.username);

    let mut user2: User = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    println!("user email: {}", user2.email);

    let user3: User = build_user("test@gmail.com".to_string(), "myuser".to_string());

    let user4: User = User {
        active: user3.active,
        username: user3.username,
        email: String::from("test2@gmail.com"),
        sign_in_count: user3.sign_in_count
    };

    println!("user email: {}", user4.email);

}

