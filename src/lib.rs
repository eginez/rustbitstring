use std::ops::{Index, Range};

pub struct Bitstring {
    // Packs the strings into bits to
    // and operates on the bits directly
    data: Vec<u8>,
    length: usize,
}

impl Bitstring {
    pub fn from_string(data: &str) -> Self {
        let len = data.chars().count();
        let capacity = (len + 7) / 8;
        println!(
            "creating an array of size {:?}, len: {:?}",
            capacity,
            len / 8
        );

        let mut data_vec = vec![0u8; capacity];
        for (pos, char) in data.char_indices() {
            let byte_index = pos / 8;
            let bit_index = pos % 8;
            let bit = match char {
                '0' => 0,
                '1' => 1,
                _ => panic!("Unexpected character '{}' at position {}", char, pos),
            };
            data_vec[byte_index] |= bit << bit_index;
        }

        Bitstring {
            data: data_vec,
            length: len,
        }
    }

    pub fn slice(&self, index: Range<usize>) -> String {
        assert!(index.end <= self.length);

        let mut res = String::from("");
        for i in index {
            res.push(self[i])
        }
        res
    }
}

impl Index<usize> for Bitstring {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        let bit = (self.data[index / 8] >> (index % 8)) & 1;
        if bit == 0 {
            &'0'
        } else {
            &'1'
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn create(#[values("1010", "11001", "100111", "111111111")] input: &str) {
        let bs = Bitstring::from_string(input);
        assert!(!bs.data.is_empty() && bs.data.len() <= 2);
        assert!(bs.length == input.chars().count());
        for (pos, ch) in input.char_indices() {
            assert!(ch == bs[pos]);
        }
    }

    #[rstest]
    fn range(#[values("1010")] input: &str) {
        let bs = Bitstring::from_string(input);
        let v = &bs.slice(1..4);
        assert!(v == "010");
        assert!("10" == &bs.slice(0..2));
    }
}
