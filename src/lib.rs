struct Bitstring {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn create(#[values("1010", "11001", "000111", "111111111")] input: &str) {
        let bs = Bitstring::from_string(input);
        println!("{:?}", bs.data);
        assert!(bs.data.len() >= 1 && bs.data.len() <= 2);
        assert!(bs.length == input.chars().count());
    }
}
