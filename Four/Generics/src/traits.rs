pub struct NewsArticle{
    pub author:String,
    pub headline:String,
    pub content:String,
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub retweet:bool,
    pub reply:bool,
}
impl Summary for Tweet{
    fn summarize(&self)->String{
        println!("{}:{}",self.username,self.content);
    }
}
impl Summary for NewsArticle{
    fn summarize(&self)->String{
        println!("{},by {}",self.headline,self.author);
    }
}
//Traits are some functionality associated with a type, for example Partial ordering copy ,etc are traits 
pub trait Summary{
    fn summarize(&self)->String;
}

fn main(){
    
}