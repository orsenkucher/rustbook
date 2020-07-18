fn main() {
    println!("Usering service started");

    let mut user = build_user(
        String::from("Orsen"),
        String::from("orsen.kucher@gmail.com"),
    );

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user
    };

    user.email = String::from("another2@example.com");

    struct Regular {
        color: Color, // lol it sees whole scope
    }
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let regular = Regular { color: black };
    // destructure and get only x
    // _ discard one value
    // .. discard everything in remainder
    let Point(x, _, ..) = origin;
    let x2 = origin.0;
    println!("{} {}", x, x2);

    struct Unit();
    let u = Unit();
}

struct User {
    username: String, // deliberately using owned type
    email: String,    // not &ref one. To use it we need lifetimes
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
