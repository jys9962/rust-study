```rust
struct Rectangle {
    width: uint32,
    length: uint32,
}

impl Rectangle {
    // &self 를 갖고있는 메서드
    fn area(&self) -> uint32 {
        self.width * self.length
    }

    // self 가 없는 연관함수
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { length: 30, width: 50 };
    let rect2 = Rectangle::square(20);

    rect1.area();
    rect2.area();
}
```
