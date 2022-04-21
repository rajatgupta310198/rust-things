use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn greet() {
    println!("Greet")
}
fn increament(mut y: u32) -> u32 {
    y = y + 1;
    y
}
fn main() {
    let g = rand::thread_rng().gen_range(1..20);
    println!("Guess tshe string 1.0 - {}", g);
    greet();
    let x = increament(g);
    println!("Value of x {}", x);
    println!("Value of g {}", g);
    let z: u32 = if g % 2 == 0 { 2 } else { 3 };
    println!("z = {}", z);
    loop {
        let mut guess = String::new();
        println!("Enter string:");
        io::stdin().read_line(&mut guess).expect("Failed");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&g) {
            Ordering::Equal => {
                println!("Great");
                break;
            }
            Ordering::Greater => println!("Bigger"),
            Ordering::Less => println!("Less"),
        }
    }

    for number in 0..10 {
        println!("Number - {}", number);
    }
    let mut s = String::from("Hello ");
    s.push_str(" World");

    println!("{}", s);
    take_ownership_str(s);
    // will not work as s becomes shallow copy
    // y will be refering to memory space where data is deleted as take_ownership_str was the owner and scope ended
    //
    // let y = s;
    // println!("{}", y);

    let x = 5;
    take_ownership_i32(x);
    // works because data types are copied
    let y = x;

    println!("{}", y);

    let mut s = String::from("Hello ");
    s.push_str(" World");
    s = take_ownership_but_return_str(s);
    // works
    let y = s;
    let size_y = get_size(&y);
    println!("{} has size of y passed by ref {}", y, size_y);

    let (size_x, size_y) = get_size_v2(&y);

    println!("{} has size of y passed by ref {}", size_x, size_y);
}

fn take_ownership_str(y: String) {
    println!("{}", y);

    // out of scope
}

fn take_ownership_i32(y: i32) {
    println!("{}", y);

    // out of scope
}

fn take_ownership_but_return_str(y: String) -> String {
    println!("{}", y);
    y
    // out of scope
}
// pass the ref so that owner is outer scope
fn get_size(y: &String) -> usize {
    let bytes = y.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    y.len()
}

fn get_size_v2(y: &String) -> (usize, usize) {
    let hello = &y[0..5];
    let world = &y[6..11];
    println!("Getsizev2");
    println!("{} ", hello);
    println!("{} ", world);
    (hello.len(), world.len())
}
