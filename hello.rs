


fn main() {

    println!("Hello, Wordl! , vim");

    let mut a = [1, 2, 3, 4, 0, 3, 5];
    let size = a.len();
    for i in 0..size {

        for j in 0..size - 1 {

            if a[j] > a[j+1] {
                let temp = a[j];
                a[j] = a[j+1];
                a[j+1] = temp;
            } 
        }
    }

    for i in 0..a.len() {
        println!("{}", a[i]);
    }
}
