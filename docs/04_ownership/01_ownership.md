```rust
// copy 불가능한 타입이 다른 변수에 move 되는 경우

fn main() {
    let s1 = String::from("aa");
    
    // s1 이 해당 시점에 해제 (drop)
    let s2 = s1;
}
```

```rust
fn main() {
    let s1 = String::from("aa");
    
    // some_function 종료 시점에 파라미터가 drop 되기 때문에
    // s1은 호출시점에 drop
    some_function(s1);
}
```




### copy 가능한 타입
- 모든 정수형 타입(i8, i32, u32 ...)
- boolean
- 모든 부동소수 타입 (f64 ...)
- copy 가능한 타입으로 구성된 튜플


