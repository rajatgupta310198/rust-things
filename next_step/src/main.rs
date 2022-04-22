struct User {
    name: String,
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}
fn main() {
    println!("Hello, world!");

    let mut user = User {
        email: String::from("rajatgupta310198@gmail.com"),
        name: String::from("Rajat Gupta"),
        active: true,
        username: String::from("rajatgupta310198"),
        sign_in_count: 1,
    };

    user.email = String::from("rajat310198@outlook.com");
    // println!("{}", user);
    let mut user2 = build_user(user.name, user.email, String::from("rajatgupta310198"));

    let user3 = User {
        email: String::from("rajat310198@outlook.com"),
        ..user2
    };
    // will not work as ownership goes away of name field =
    // println!("{}", user2.name);
    // shallow copy
    let mut x = some_new();
    println!("{}", x);
    let mut a = [1, 2, 3, 4, 5, 0];
    let size = a.len();
    println!("Size of {}", size);
    for i in 0..size {
        for j in 0..size - 1 {
            if a[j] > a[j + 1] {
                let temp = a[j];
                a[j] = a[j + 1];
                a[j + 1] = temp;
            }
        }
        println!("{}", a[i])
    }

    for i in 0..size {
        println!("{}", a[i])
    }
}
fn build_user(name: String, email: String, username: String) -> User {
    User {
        name: name,
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn some_new() -> i32 {
    let y = 1; // stack
    y
}
