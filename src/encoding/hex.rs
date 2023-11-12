use crate::encoding::EncodingType;

pub struct Hex {}

impl EncodingType for Hex {
    #[inline]
    fn decode(input: &str) -> Option<Vec<u8>>  {
        input.chars()
            .array_chunks::<2>()
            .map(|[c1, c2]| {
                if !c1.is_ascii_hexdigit() || !c2.is_ascii_hexdigit() {
                    return None;
                }
                
                if let (Some(b1), Some(b2)) = (c1.to_digit(16), c2.to_digit(16)) {
                    return Some((b1 as u8) << 4 | (b2 as u8));
                }
                return Some(0);
            })
            .collect()
        // Infer converting Vec<Option<T>> to Option<Vec<T>> using the
        // return type.
    }

    #[inline]
    fn encode(input: &Vec<u8>) -> String {
        input.iter()
            .map(|x| {
                let high = ((x & 0xf0) >> 4) as u32;
                let low = (x & 0x0f) as u32;

                // We're guaranteed that these are valid hex anyways
                // let's use unwrap.
                vec![std::char::from_digit(high, 16).unwrap(),
                     std::char::from_digit(low, 16).unwrap()]
            })
            .flatten()
            .collect()
    }
}
