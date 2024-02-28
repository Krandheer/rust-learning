pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} {}", self.headline, self.author, self.content)
    }
}

fn main() {
    let news = NewsArticle{
        headline: String::from("some one did some amazing things"),
        author: String::from("randheer"),
        content: String::from("there is some amazing thing being done by indian customer on purpose and that's quite impressing"),
        location: String::from("sultanpur"),
    };
    println!("1 new article, {:?}", news.summarize());
}
