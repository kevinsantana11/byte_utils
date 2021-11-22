use byte_utils;

#[test]
fn test_add_bytes() {
    let b1 = byte_utils::Bytes::new(vec![1]);
    let b2 = byte_utils::Bytes::new(vec![2]);
    let res = b1 + b2;
    assert!(res.is_equal(&byte_utils::Bytes::new(vec![3])))
}