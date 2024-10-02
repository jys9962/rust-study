열거형
```rust
enum IpAddrKind {
    V4,
    V6,
}

fn route(addr: IpAddrKind) {}

fn main() {
    route(IpAddrKind::V4);
}

```


열거형에 데이터 추가
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```


열거형에 메서드
```rust

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let message = Message::Quit;
    message.call()
}



```
