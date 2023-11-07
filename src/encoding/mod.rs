pub trait Encoding {
    fn decode(input: &str) -> Option<Vec<u8>>;
    fn encode(input: &Vec<u8>) -> String;
}

pub struct Base64 {}
pub struct Hex {}

#[inline]
fn base64_get_char(b: u8) -> char {
    let alphabet =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    std::char::from_u32(alphabet[b as usize] as u32).unwrap()
}

impl Encoding for Base64 {
    fn decode(_input: &str) -> Option<Vec<u8>> {
        None
    }

    fn encode(input: &Vec<u8>) -> String {
        input.iter()
            .array_chunks::<3>()
            .map(|[b1, b2, b3]| {
                let c1 = base64_get_char((b1 >> 2) & 0x1F);
                let c2 =
                    base64_get_char(((b1 & 0x3) << 4) |
                                    ((b2 & 0xF0) >> 4 ));
                let c3 =
                    base64_get_char(((b2 & 0x0F) << 2) |
                                    ((b3 & 0xC0) >> 6));
                let c4 = base64_get_char(b3 & 0x3F);
                
                vec![c1, c2, c3, c4]
            })
            .flatten()
            .collect()
    }
}

impl Encoding for Hex {
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
