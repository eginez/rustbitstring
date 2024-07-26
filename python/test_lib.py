import unittest

import packed_bitstring as pb
from packed_bitstring import Bitstring as BS


class TestBitString(unittest.TestCase):
    def test_create(self) -> None:
        bs = pb.Bitstring("10")
        assert bs.get_bit(0) == 1

    def test_str(self) -> None:
        string = "10010010011111001010010"
        bs = pb.Bitstring(string)
        assert string == str(bs)

    def test_reverse(self) -> None:
        string = "10010010011111001010010"
        rev = string[::-1]
        bs = pb.Bitstring(string)
        assert rev == str(bs.reverse())

    def test_slice(self) -> None:
        string = "101000"
        bs = pb.Bitstring(string)
        self.assertEqual(1, bs[2])

    def test_in_equality(self) -> None:
        self.assertEqual(BS("1"), BS("1"))

    def test_in_dict(self) -> None:
        dd = {BS("1"): "one"}
        self.assertEqual("one", dd[BS("1")])



if __name__ == "__main__":
    unittest.main()
