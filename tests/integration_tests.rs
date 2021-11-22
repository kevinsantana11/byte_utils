use byte_utils;

#[test]
fn test_add_bytes() {
    let b1 = byte_utils::Bytes::new(vec![1]);
    let b2 = byte_utils::Bytes::new(vec![2]);
    let res = b1 + b2;
    assert!(res.is_equal(&byte_utils::Bytes::new(vec![3])))
}

#[test]
fn test_mult_bytes() {
    let b1 = byte_utils::Bytes::new(vec![4]);
    let b2 = byte_utils::Bytes::new(vec![2]);
    let res = b1 * b2;
    assert!(res.is_equal(&byte_utils::Bytes::new(vec![8])))

}

#[test]
fn test_pow_bytes() {
    let b2 = byte_utils::Bytes::new(vec![2]);
    let res = b2.pow(byte_utils::Bytes::new(vec![4]));
    assert!(res.is_equal(&byte_utils::Bytes::new(vec![16])))
}

#[test]
fn test_is_equal() {
    let b1 = byte_utils::Bytes::new(vec![4, 4]);
    let b2 = byte_utils::Bytes::new(vec![4, 4]);
    assert!(b1.is_equal(&b2))
}