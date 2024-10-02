스트링은 바이트 벡터의 래퍼로 구현


```rust 
fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s: String = data.to_string();
    let s: String = String::from(data);
    
}
```


format 사용
```rust

fn main() {
    let s1 = "tic";
    let s2 = "tac";
    let s3 = "to";
    
    let s = format!("{s1}-{s2}-{s3}");
}
```


반복문
```rust
fn main() {
    for c in "Зд".chars() {
        println!("{c}");
    }
    // З
    // д


    for b in "Зд".bytes() {
        println!("{b}");
    }
    // 208
    // 151
    // 208
    // 180
}

```
