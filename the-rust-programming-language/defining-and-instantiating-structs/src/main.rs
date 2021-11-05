fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername"),
    );
    let user2 = User {
        username: String::from("anotherusername"),
        ..user1
    };
    
    println!("user1's username is {}", user1.username);
    println!("user2's username is {}", user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
