struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {

    let user1 = build_user("david@gmail.com".to_string(), "davidhuang".to_string());

    // user1 cannot be used after because ownership transferred to user2
    // if user2 created new Strings for email AND username, only then user1 can be used
    // because of the "Copy" trait (for numbers and not Strings)
    let user2 = User {
        email: String::from("second@gmail.com"),
        ..user1
    };

    println!("{0}", user1.email);
    println!("{0}", user2.email); 
}


