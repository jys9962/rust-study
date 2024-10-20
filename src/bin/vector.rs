fn main() {
    let mut list: Vec<i32>;

    // 빈 벡터
    list = Vec::new();

    // 미리 사이즈 점유
    list = Vec::with_capacity(100);

    // [1,2,3] 초기화
    list = vec![1, 2, 3];
    list = (1..4).collect();
    list = (1..=3).collect();

    // 100 값으로 50건
    list = vec![100; 50];

    a = list.select_nth_unstable(10);
}
