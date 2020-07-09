fn main() {
    println!("Usering service started");

    let user = build_user(
        String::from("Orsen"),
        String::from("orsen.kucher@gmail.com"),
    );
}

struct User {
    username: String,
    email: String,
    sing_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sing_in_count: 1,
    }
}
