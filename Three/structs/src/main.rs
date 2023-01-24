// use core::str;
// #[derive(Debug)]
// struct User{
//     username:String,
//     email:String,
//     sign_in_count:u32,
//     active :bool,

// }
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height:u32,
}
impl Rectangle {
    fn area(&self)->u32{
        return self.height*self.width;
    }
    fn can_hold(&self,other:Rectangle)->bool{
        return self.width>other.width && self.height>other.height;
    }
}
fn main() {
    let rect = Rectangle{
        height : 15,
        width:32
    };
    let rect2 = Rectangle{
        height:100,
        width:30
    };

    println!("Area 1 is {:#?}",rect.area());
    println!("{:#?}",rect.can_hold(rect2));

}
//     let mut user1 = User{
//         email:("1@gmail.com").to_string(),
//         username:("1").to_string(),
//         sign_in_count:1,
//         active:true
//     };
//     let name = user1.username;
//     user1.username = String::from(name);
//     let user2 = build_user(("2@gmail.com").to_string(),String::from("Ram"));
//     let user3 = User{
//         username:"Ram2".to_string(),
//         email:String::from("3.email.com"),
//         ..user2
//     };
//     println!("{:#?}",user3);

// }
// fn build_user(email:String,name:String)->User{
//     return User{
//         email:email,
//         username:name,
//         sign_in_count:1,
//         active:true
//     };
// }
    // let mut user1 = User{
    //     email:("1@gmail.com").to_string(),
    //     username:("1").to_string(),
    //     sign_in_count:1,
    //     active:true
    // };
    // let name = user1.username;
    // user1.username = String::from(name);
    // let user2 = build_user(("2@gmail.com").to_string(),String::from("Ram"));
    // let user3 = User{
    //     username:"Ram2".to_string(),
    //     email:String::from("3.email.com"),
    //     ..user2
    // };
    // println!("{:#?}",user3);
// fn build_user(email:String,name:String)->User{
//     return User{
//         email:email,
//         username:name,
//         sign_in_count:1,
//         active:true
//     };
// }
//     let mut user1 = User{
//         email:("1@gmail.com").to_string(),
//         username:("1").to_string(),
//         sign_in_count:1,
//         active:true
//     };
//     let name = user1.username;
//     user1.username = String::from(name);
//     let user2 = build_user(("2@gmail.com").to_string(),String::from("Ram"));
//     let user3 = User{
//         username:"Ram2".to_string(),
//         email:String::from("3.email.com"),
//         ..user2
//     };
//     println!("{:#?}",user3);

// }
// fn build_user(email:String,name:String)->User{
//     return User{
//         email:email,
//         username:name,
//         sign_in_count:1,
//         active:true
//     };
// }
//     let mut user1 = User{
//         email:("1@gmail.com").to_string(),
//         username:("1").to_string(),
//         sign_in_count:1,
//         active:true
//     };
//     let name = user1.username;
//     user1.username = String::from(name);
//     let user2 = build_user(("2@gmail.com").to_string(),String::from("Ram"));
//     let user3 = User{
//         username:"Ram2".to_string(),
//         email:String::from("3.email.com"),
//         ..user2
//     };
//     println!("{:#?}",user3);

// }
// fn build_user(email:String,name:String)->User{
//     return User{
//         email:email,
//         username:name,
//         sign_in_count:1,
//         active:true
//     };
// }
