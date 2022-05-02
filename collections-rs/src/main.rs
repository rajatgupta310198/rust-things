use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut s = "HELLO".to_string();
    s.push_str(" \nstring");

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    println!("{}", s3);

    // fn add(self, s: &str) +
    // println!("{}", s1);

    // add does not take ownership of &str
    // self does not have & i.e reference

    // ref is added to s1
    println!("{}", s2);

    println!("{}", &s2[0..4]);

    for b in s2.as_bytes() {
        println!("{}", b);
    }

    let mut sample: HashMap<String, i32> = HashMap::new();
    sample.insert("Hello".to_string(), 10);
    sample.insert("World".to_string(), 20);

    dbg!(sample);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // won't work
    // for i in &teams {
    //     dbg!(i);
    // }

    // dbg!(scores);

    scores.entry("key".to_string()).or_insert(24);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
