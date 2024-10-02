해시맵 기본
```rust 
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```


해시 반복문
```rust 
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
```

값 추가하기
```rust 
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // 값 덮어쓰기
    scores.insert(String::from("Blue"), 20);

    // 없을때만 쓰기
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
```
