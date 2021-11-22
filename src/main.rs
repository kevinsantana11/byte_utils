mod byte_utils;


fn main() {
    let b1 = byte_utils::Bytes::new(vec![2]);

    let res = b1.pow(byte_utils::Bytes::new(vec![8]));
    println!("Hello, world!, res is: {:?}", res);
}
