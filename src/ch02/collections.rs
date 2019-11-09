/// # 线性序列： 向量(Vec)
/// 
/// Basic usage:
/// 
/// ```
/// fn vec_example() {
///     let mut v1 = vec![];
///     v1.push(1); // 使用push方法插入数据
///     v1.push(2);
///     v1.push(3);
///     assert_eq!(v1, [1,2,3]);
///     assert_eq!(v1[1], 2);
///     let mut v2 = vec!["hello".to_string(); 10];  // 像数组那样初始化
///     assert_eq!(v2.len(), 10);
///     assert_eq!(v2[5], "hello".to_string());
///     let mut v3 = Vec::new();
///     v3.push(4);
///     v3.push(5);
///     v3.push(6);
///     // v3[4];  error: index out of bounds
/// }
/// ```
pub fn vec_example() {
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);

    assert_eq!(v1, [1,2,3]);
    assert_eq!(v1[1], 2);

    let v2 = vec!["hello".to_string(); 10];

    assert_eq!(v2.len(), 10);
    assert_eq!(v2[5], "hello".to_string());
    let mut v3 = Vec::new();

    v3.push(4);
    v3.push(5);
    v3.push(6);
}

/// # 线性序列: 双向队列(VecDeque)
/// 
/// Basic usage:
/// 
/// ```
/// use std::collections::VecDeque;
/// fn vec_deque(){
///     let mut buf = VecDeque::new();
/// 
///     buf.push_front(1);
///     buf.push_front(2);
///     assert_eq!(buf.get(0), Some(&2));
///     assert_eq!(buf.get(1), Some(&1));
/// 
///     buf.push_back(3);
///     buf.push_back(4);
///     buf.push_back(5);
/// 
///     assert_eq!(buf.get(2), Some(&3));
///     assert_eq!(buf.get(3), Some(&4));
///     assert_eq!(buf.get(4), Some(&5));
/// }
/// vec_deque();
/// ```
pub fn vec_deque(){
    use std::collections::VecDeque;
    let mut buf = VecDeque::new();

    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0),Some(&2));
    assert_eq!(buf.get(1), Some(&1));

    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);

    assert_eq!(buf.get(2), Some(&3));
    assert_eq!(buf.get(3), Some(&4));
    assert_eq!(buf.get(4), Some(&5));
}