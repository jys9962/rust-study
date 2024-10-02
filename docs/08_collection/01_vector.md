vec 기본
```rust

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v = vec![1, 2, 3];
}
```


```rust
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    // 없는 경우 panic
    let third = &v[10];

    // option 
    let third: Option<&i32> = v.get(2);
    println!("the third element is {third}");
    
    // v와 third 모두 drop
}

```
