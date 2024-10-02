불변 참조자를 이용한 함수 호출
```rust
fn main() {
    let s1 = String::from("hello");
    
    // 참조를 넘기기 때문에 s1이 drop 되지 않음
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// string 의 참조를 입력받음
fn calculate_length(s: &String) -> usize {
    s.len()
}
// s 에 대한 소유권이 없으므로 drop 하지 않음
```

가변 참조자를 이용한 함수 호출
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    // 가변 참조자를 입력받았으므로 변수를 수정할 수 있다
    some_string.push_str(", world");
}
```


동일 변수에 대해 불변/가변 참조자를 같이 쓰는 경우 오류
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    // 불변 참조자가 있는 경우 가변참조자 만들 수 없음
    let r3 = &mut s;
}
```
