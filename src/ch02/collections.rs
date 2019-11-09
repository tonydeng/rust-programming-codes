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

/// # 线性序列：链表（LinkedList）
///
/// Basic usage:
///
/// ```
/// use std::collections::LinkedList;
/// fn linked_list() {
///     let mut list1 = LinkedList::new();
///
///     list1.push_back('a');
///
///     let mut list2 = LinkedList::new();
///     list2.push_back('b');
///     list2.push_back('c');
///
///     list1.append(&mut list2);
///     println!("{:?}", list1); // ['a', 'b', 'c']
///     println!("{:?}", list2); // []
///
///     list1.pop_front();
///     println!("{:?}", list1); // ['b', 'c']
///
///     list1.push_front('e');
///     println!("{:?}", list1); // ['e', 'b', 'c']
///
///     list2.push_front('f');
///     println!("{:?}", list2); // ['f']
/// }
/// linked_list();
pub fn linkd_list() {
    use std::collections::LinkedList;
    let mut list1 = LinkedList::new();

    list1.push_front('a');

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list1.append(&mut list2);
    println!("{:?}", list1);    // ['a', 'b', 'c']
    println!("{:?}", list2);    // []

    list1.pop_front();
    println!("{:?}", list1); // ['b', 'c']

    list1.push_front('e');
    println!("{:?}", list1); // ['e','b','c']

    list2.push_front('f');
    println!("{:?}", list2); // ['f']
}

/// # Key-Value映射表: HashMap和BTreeMap
///
/// Basic usage:
///
/// ```
/// use std::collections::BTreeMap;
/// use std::collections::HashMap;
/// fn map_demo() {
///     let mut hmap = HashMap::new();
///     let mut bmap = BTreeMap::new();
///
///     hmap.insert(3, "c");
///     hmap.insert(1, "a");
///     hmap.insert(2, "b");
///     hmap.insert(5, "e");
///     hmap.insert(4, "d");
///
///     bmap.insert(3, "c");
///     bmap.insert(2, "b");
///     bmap.insert(1, "a");
///     bmap.insert(5, "e");
///     bmap.insert(4, "d");
///
///      // 输出结果为：{1: "a", 2: "b", 3: "c", 5: "e", 4: "d"}，但key的顺序是随机的，因为HashMap是无序的
///     println!("{:?}", hmap);
///     // 输出结果永远都是 {1: "a", 2: "b", 3: "c", 4: "d", 5: "e"}，因为BTreeMap是有序的
///     println!("{:?}", bmap);
/// }
/// map_demo();
/// ```
pub fn map_demo() {
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    let mut hmap = HashMap::new();
    let mut bmap = BTreeMap::new();

    hmap.insert(3, "c");
    hmap.insert(1, "a");
    hmap.insert(2, "b");
    hmap.insert(5, "e");
    hmap.insert(4, "d");

    bmap.insert(3, "c");
    bmap.insert(2, "b");
    bmap.insert(1, "a");
    bmap.insert(5, "e");
    bmap.insert(4, "d");
    // 输出结果为：{1: "a", 2: "b", 3: "c", 5: "e", 4: "d"}，但key的顺序是随机的，因为HashMap是无序的
    println!("{:?}", hmap);
    // 输出结果永远都是 {1: "a", 2: "b", 3: "c", 4: "d", 5: "e"}，因为BTreeMap是有序的
    println!("{:?}", bmap);
}