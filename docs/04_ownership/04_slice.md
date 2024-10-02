슬라이스를 사용하지 않아 문제가 있는 코드
```rust
fn first_word() -> usize {
    //     
}

fn main() {
    let mut s = String::from("hello world");
    
    // s 와 관련된 word
    let word = first_word(&s);

    s.clear(); 

    
    // word 는 여기서 사용할 수 있지만 s는 비워짐 
}
```


슬라이스 사용
```rust
fn first_word(s: &String) -> &str {
    // ...
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // 불변 참조인 word 가 있는 상태에서 
    // s 에 대한 가변참조를 만들기 때문에 오류 발생
    s.clear(); // Error!

    println!("the first word is: {}", word);
}
```
