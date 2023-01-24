enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String), 
}
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
fn main() {
    // let four = IpAddr::V4;
    // let six = IpAddr::V6;
    let x: i8=5;
    let y = Some(5);
    let sum = x+y.unwrap_or(0);
    println!("{}", sum);

}
