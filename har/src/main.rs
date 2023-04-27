mod huffman;

fn main() {
    let mut frequencies = [0; 256];
    frequencies['f' as usize] = 5;
    frequencies['e' as usize] = 9;
    frequencies['c' as usize] = 12;
    frequencies['b' as usize] = 13;
    frequencies['d' as usize] = 16;
    frequencies['a' as usize] = 45;
    println!("frequencies: {frequencies:?}");
    let codes = huffman::huffman(&frequencies);
    println!("codes: {codes:?}");

    println!("Hello, world!");

    let mut frequencies = [0; 256];
    frequencies['f' as usize] = 5;
    frequencies['e' as usize] = 9;
    frequencies['c' as usize] = 12;
    frequencies['b' as usize] = 13;
    frequencies['d' as usize] = 16;
    frequencies['a' as usize] = 45;
    let codes = huffman::huffman(&frequencies);
    assert_eq!(format!("{:?}", codes['a' as usize]), "0");
    assert_eq!(format!("{:?}", codes['b' as usize]), "101");
    assert_eq!(format!("{:?}", codes['c' as usize]), "100");
    assert_eq!(format!("{:?}", codes['d' as usize]), "111");
    assert_eq!(format!("{:?}", codes['e' as usize]), "1101");
    assert_eq!(format!("{:?}", codes['f' as usize]), "1100");

    assert_eq!(huffman::huffman_encode(&codes, "abc".as_bytes()), (vec![0b01011000], 1));
    assert_eq!(huffman::huffman_decode(&codes, &[0b01011000], 1), "abc".as_bytes().to_vec());
}
