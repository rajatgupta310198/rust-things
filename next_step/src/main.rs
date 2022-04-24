struct User {
    name: String,
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Coordinates2d(i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_width(&self) -> bool {
        self.width > 0
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrs {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        dbg!(self);
    }
}

enum Option<T> {
    None,
    Some(T),
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

    let mut x_y = Coordinates2d(5, 10);
    dbg!(x_y);
    let mut a: i32 = 10;
    println!("{}", f32::sqrt(a as f32));

    println!("{}", a.abs());
    println!("{}", a.pow(2));

    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Rectangle area {}", rect1.area());
    println!("Rectangle Width {}", rect1.is_width());

    // ENUMS

    let v4 = IpAddrKind::V4;
    route(&v4);

    let home_address = IpAddress {
        kind: v4,
        address: String::from("127.0.0.1"),
    };

    dbg!(home_address);

    let home_addr = IpAddrs::V4(String::from("localhost"));

    let m = Message::Write(String::from("Hey"));
    m.call();
    let op = Some(1);
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

fn route(ip_kind: &IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => {
            println!("v4");
        }
        IpAddrKind::V6 => {
            println!("v6");
        }
    }
}
