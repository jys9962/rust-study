```rust

// 인터페이스 역할의 trait 
pub trait Summary {
    fn summarize_author(&self) -> String;

    // 기본 구현, 오버라이딩 가능l
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// NewArticle 에 대한 Summary 구현
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        todo!()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Tweet 에 대한 Summary 구현
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        todo!()
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```



매개변수로 트레이트 입력

```rust
pub fn notify(item: &impl Summary) {}
pub fn notify<T: Summary>(item: &T) {}
pub fn notify<T>(item: &T)
where
    T: Summary,
{}

```

여러 트레이트 구현한 매개변수
```rust 
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}
pub fn notify<T>(item: &T)
where
    T: Summary + Display,
{}
```
