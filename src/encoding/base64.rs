use crate::encoding::EncodingType;

pub struct Base64 {}

impl EncodingType for Base64 {
    #[inline]
    fn decode(input: &str) -> Option<Vec<u8>> {
        use base64ct::Encoding;
        base64ct::Base64::decode_vec(input).ok()
    }

    #[inline]
    fn encode(input: &Vec<u8>) -> String {
        use base64ct::Encoding;
        base64ct::Base64::encode_string(input.as_slice())
    }
}
