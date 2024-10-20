fn main() {
    let list: Vec<i32> = vec![1, 3, 5, 10, 9, 8, 2];

    // filter
    let _ = list.iter()
        .filter(|&&x| x > 5);

    // map
    let _ = list.iter()
        .map(|&x| x);

    // partition : true/false 값으로 나누기
    // (true목록, false목록)
    let (_, _): (Vec<i32>, Vec<i32>) = list.iter()
        .partition(|&&x| x % 2 == 0);


    // select_nth_unstable : 피봇인덱스 입력해서 (작은값[], 인덱스값, 큰값[]) 반환
    let mut list2 = list.clone();
    let select_result = list2.select_nth_unstable(3);
    println!("select_result = {:?}", select_result);
}
