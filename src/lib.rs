use std::cmp::max;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// I need a struct to hold a compact representation of a bitstring
// and operations on it

struct Bitstring {
    // Packs the strings into bits to
    // and operates on the bits directly
    data: Vec<u8>,
}

impl Bitstring {
    pub fn from_string(data: &str) -> Self {
        let len = data.chars().count();
        let capacity = max(1, len / 8);
        println!("creating an array of size {:?}", capacity);

        let mut current_num: u8 = 0;
        let mut data_vec = vec![0; capacity];
        for (pos, char) in data.char_indices() {
            let value = match char {
                '0' => 0,
                '1' => 1,
                _ => panic!("Unexpected character '{}' at position {}", char, pos),
            };
            current_num = current_num | (value << pos);
        }
        data_vec.push(current_num);
        Bitstring { data: data_vec }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn create() {
        let bs = Bitstring::from_string("0110");
        println!("{:?}", bs.data);
        assert!(!bs.data.is_empty());
    }
}
