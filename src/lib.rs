use std::{
    fmt::{Display, Formatter, Result},
    ops::{Index, Range},
};

#[derive(Debug)]
pub struct Bitstring {
    // Packs the strings into bits to
    // and operates on the bits directly
    data: Vec<u8>,
    // The number of characters in the bitstring
    length: usize,
}

impl Display for Bitstring {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.slice(0..self.length))
    }
}

impl Bitstring {
    pub fn from_string(data: &str) -> Self {
        let len = data.chars().count();
        let capacity = (len + 7) / 8;
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

    fn _calculate_byte_bit(&self, index: usize) -> Option<(usize, u8)> {
        if index >= self.length {
            return None;
        }
        Some((index / 8, (index % 8) as u8))
    }

    pub fn slice(&self, index: Range<usize>) -> String {
        assert!(index.end <= self.length);

        let mut res = String::from("");
        for i in index {
            res.push(self[i])
        }
        res
    }

    pub fn reverse(&self) -> Bitstring {
        let mut temp = vec![0u8; self.data.len()];
        for i in (0..self.length).rev() {
            let bit = self.get_bit(i);
            let (byte_index, bit_index) = self._calculate_byte_bit(self.length - i - 1).unwrap();
            temp[byte_index] |= bit << bit_index;
        }
        Bitstring {
            data: temp,
            length: self.length,
        }
    }

    pub fn get_bit(&self, index: usize) -> u8 {
        let (byte, bit) = self._calculate_byte_bit(index).unwrap();
        (self.data[byte] >> bit) & 1
    }
}

impl Index<usize> for Bitstring {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        if self.get_bit(index) == 0 {
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

    #[rstest]
    #[case("1", "1")]
    #[case("10", "01")]
    #[case("0011", "1100")]
    #[case(
        "100111011101000101001010001100101",
        "101001100010100101000101110111001"
    )]
    fn reverse(#[case] input: &str, #[case] expected: &str) {
        let bs = Bitstring::from_string(input);
        assert!(expected == format!("{}", bs.reverse()));
    }
}
