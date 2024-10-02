
루트 파일(주로 src/main.rs or src/lib.rs) 에는 모듈을 선언할 수 있다

```rust

// src/garden.rs
// src/garden/mod.rs
mod garden;

```


크레이트 루트가 아닌 다른 파일에서는 서브모듈을 선언할 수 있다  
src/garden.rs
```rust

// src/garden/vegetables.rs
// src/garden/vegetables/mod.rs
mod vegetables;

```
