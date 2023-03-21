mod huffman;

fn main() {
    let mut frequencies = [0; 256];
    frequencies['f' as usize] = 5;
    frequencies['e' as usize] = 9;
    frequencies['c' as usize] = 12;
    frequencies['b' as usize] = 13;
    frequencies['d' as usize] = 16;
    frequencies['a' as usize] = 45;
    println!("frequencies: {:?}", frequencies);
    let codes = huffman::huffman(&frequencies);
    println!("codes: {:?}", codes);

    println!("Hello, world!");
}
