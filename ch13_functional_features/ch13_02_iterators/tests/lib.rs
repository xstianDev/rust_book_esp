#[test]
fn iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn into_iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.into_iter();
    
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iter_sum_demo() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}