use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default)]
struct Node {
    symbol: Option<u8>,
    frequency: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
pub struct Code(u16, usize); // code as bits and its length

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.1 != 0 {
            return write!(f, "{:0>width$b}", self.0, width = self.1);
        }
        Ok(())
    }
}

pub fn huffman(frequencies: &[usize; 256]) -> [Code; 256] {
    let root = build_huffman_tree(frequencies);
    construct_huffman_code(root)
}

fn build_huffman_tree(frequencies: &[usize; 256]) -> Option<Box<Node>> {
    let nodes = frequencies
        .iter()
        .enumerate()
        .filter(|(_, frequency)| **frequency != 0)
        .map(|(symbol, frequency)| {
            Box::new(Node { symbol: Some(symbol as u8), frequency: *frequency, ..Default::default() })
        })
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::from(nodes);

    while heap.len() >= 2 {
        let x = heap.pop().unwrap();
        let y = heap.pop().unwrap();

        let frequency = x.frequency + y.frequency;
        let z = Box::new(Node { left: Some(x), right: Some(y), frequency, ..Default::default() });
        heap.push(z);
    }

    heap.pop()
}

fn construct_huffman_code(root: Option<Box<Node>>) -> [Code; 256] {
    let mut codes = HashMap::new();
    if let Some(root) = root {
        construct_huffman_code_helper(&root, Code(0, 0), &mut codes);
    }
    let mut codes2 = [Code(0, 0); 256];
    for (k, v) in codes {
        codes2[k as usize] = v;
    }
    codes2
}

fn construct_huffman_code_helper(node: &Node, prefix: Code, codes: &mut HashMap<u8, Code>) {
    if let Some(symbol) = node.symbol {
        println!("symbol {}  prefix {:?}", symbol as char, prefix);
        codes.insert(symbol, prefix);
        return;
    }

    if let Some(left) = &node.left {
        let mut prefix = prefix;
        prefix.0 <<= 1;
        prefix.1 += 1;
        construct_huffman_code_helper(left, prefix, codes);
    }
    if let Some(right) = &node.right {
        let mut prefix = prefix;
        prefix.0 <<= 1;
        prefix.0 |= 1;
        prefix.1 += 1;
        construct_huffman_code_helper(right, prefix, codes);
    }
}

pub fn huffman_encode(codes: &[Code; 256], src: &[u8]) -> (Vec<u8>, usize) {
    let mut result = vec![];
    let mut d = 0_u8;
    let mut l = 0_usize;

    for a in src {
        let Code(mut bits, mut length) = codes[*a as usize];

        while length > 0 {
            let aaa = cmp::min(length, 8 - l);
            let x = bits >> (length - aaa);
            let y = x << (8 - l - aaa);
            d |= y as u8;
            l += aaa;

            if l == 8 {
                result.push(d);
                d = 0;
                l = 0;
            }

            length -= aaa;
            bits = bits << ((16 - length) % 16) >> ((16 - length) % 16);
        }
    }

    if l == 0 {
        (result, 0)
    } else {
        result.push(d);
        (result, 8 - l)
    }
}

pub fn huffman_decode() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(format!("{:?}", Code(0b00, 2)), "00");
        assert_eq!(format!("{:?}", Code(0b01, 2)), "01");
        assert_eq!(format!("{:?}", Code(0b101, 3)), "101");

        let mut frequencies = [0; 256];
        frequencies['f' as usize] = 5;
        frequencies['e' as usize] = 9;
        frequencies['c' as usize] = 12;
        frequencies['b' as usize] = 13;
        frequencies['d' as usize] = 16;
        frequencies['a' as usize] = 45;
        let codes = huffman(&frequencies);
        assert_eq!(format!("{:?}", codes['a' as usize]), "0");
        assert_eq!(format!("{:?}", codes['b' as usize]), "101");
        assert_eq!(format!("{:?}", codes['c' as usize]), "100");
        assert_eq!(format!("{:?}", codes['d' as usize]), "111");
        assert_eq!(format!("{:?}", codes['e' as usize]), "1101");
        assert_eq!(format!("{:?}", codes['f' as usize]), "1100");

        assert_eq!(huffman_encode(&codes, "abc".as_bytes()), (vec![0b01011000], 1));
    }
}
