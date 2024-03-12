fn main() {
    let  v: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32>= v.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
