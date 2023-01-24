struct Point<T,U>{
    x: T,
    y: U 

}

fn main() {
    let number_list = vec![100,102,104,111,154];
    let largest = get_max(number_list);
    println!(
        "{}",largest
    );
}
fn get_max<T: PartialOrd + Copy>(number_list:Vec<T>)->T {
    let mut largest = number_list[0];
    
    for i in number_list{
        if i > largest {
            largest = i;
        }
        
    }
    return largest;
    
}