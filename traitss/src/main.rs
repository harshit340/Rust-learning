pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}: {}", self.headline , self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}
// this is a default implementation of traits
pub trait Summary {
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}

//Traits as Parameters

//type 1
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//type 2 trait bound syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// some more example

pub fn notify(item1:&(impl Summary+Display), items2:&impl Summary){
    //...
}
pub fn notify<T:Summary+Display>(item1:&T , items2:&T){
    //...
}




fn main(){
    let tweet = Tweet{
        username:String::from("@harshit"),
        content:String::from("Hello world"),
        reply:false,
        retweet:false
    };

    let article = NewsArticle{
         headline: String::from("Harshit shrivastava"),
         author: String::from("The Sky is Falling!"),
         content: String::from("The Sky is not actually falling")
    };

    println!("Tweet summary:{}",tweet.summarize());
    println!("article summary:{}",article.summarize());

}
