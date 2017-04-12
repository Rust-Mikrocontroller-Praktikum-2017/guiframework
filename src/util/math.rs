pub fn isqrt(num: u32) -> u32 {
    let mut num: u32 = num;
    let mut res: u32 = 0;
    let mut bit: u32 = 1 << 30;

    while bit > num {
        bit >>= 2;
    }

    while bit != 0 {
        if num >= res + bit {
            num -= res + bit;
            res = (res >> 1) + bit;
        } else {
            res >>= 1;
        }
        bit >>= 2;
    }
    res
}
