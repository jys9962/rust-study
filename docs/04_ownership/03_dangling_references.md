댕글링 참조에서 오류나는 경우
```rust
fn main() {
    // 빈 값을 참조하
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    
    // s의 참조를 반환하지만 s는 함수 종료시 drop됨
    &s
}
```

