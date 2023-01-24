use std::{collections::{VecDeque, HashMap}, fmt::Debug};

fn main() {
    // let mut a = [1,2,3];
    // let mut v1 = vec![1,3,4];
    // let mut v2:Vec<i32> = Vec::new();
    // let mut v3:VecDeque<i32> = VecDeque::new();
    // v2.push(6);
    // for i in v2.iter() {
    //     println!("{}",i);
    // }  
    let text = "hello world beatiful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:?}",map);
}
