
pub fn lis(arr: &Vec<i32>) -> Vec<i32> {

    let mut res: Vec<i32> = Vec::new();
    
    
    let mut max = &arr[0];
    res.push(*max);
    for i in 1..arr.len() {

       if max < &arr[i] {
           max = &arr[i];
           res.push(*max);
       }
    }

    res


}