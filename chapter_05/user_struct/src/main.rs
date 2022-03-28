#[derive(Debug)] //打印struct 需要这个宏

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // mut 可变定义
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    println!("{:#?}", user1); //struct 打印方式

    // change user1 email
    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("someusername321");
    user1.sign_in_count = 2;
    user1.active = false;

    println!("{:#?}", user1);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
