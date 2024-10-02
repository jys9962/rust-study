라이프타임 관련 컴파일 오류

```rust
// x, y 중 어떤 값이 반환될지 알 수 없음
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

함수 시그니처에 라이프타임 명시
= 리턴값의 라이프타임은 매개변수들의 라이프타임 중 작은것과 동일함
```rust 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);


        // string2 drop
    }

    // compile error, string2 가 내부스코프에서 종료되므로 
    // result 의 라이프타임이 내부스코프에서 됨료
    print!("{result}");

    // string1 drop
}
```


구조체에서 라이프타임 명시
```rust 
// ImportantExcerpt 인스턴스는 part 참조자의 라이프타임보다 오래 살 수 없음
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```
