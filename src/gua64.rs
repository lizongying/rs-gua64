use std::collections::HashMap;
const GUA: &str = "䷁䷖䷇䷓䷏䷢䷬䷋\
䷎䷳䷦䷴䷽䷷䷞䷠\
䷆䷃䷜䷺䷧䷿䷮䷅\
䷭䷑䷯䷸䷟䷱䷛䷫\
䷗䷚䷂䷩䷲䷔䷐䷘\
䷣䷕䷾䷤䷶䷝䷰䷌\
䷒䷨䷻䷼䷵䷥䷹䷉\
䷊䷙䷄䷈䷡䷍䷪䷀";

const PADDING: char = '〇';

pub struct Gua64 {
    list: Vec<char>,
    dict: HashMap<char, u8>,
}

impl Gua64 {
    pub fn new() -> Self {
        let list: Vec<char> = GUA.chars().collect();
        let mut dict = HashMap::new();
        for (i, c) in list.iter().enumerate() {
            dict.insert(*c, i as u8);
        }
        Gua64 { list, dict }
    }

    pub fn encode(&self, input: &[u8]) -> String {
        let mut encoded = String::new();
        let mut index: u8;

        for chunk in input.chunks(3) {
            let chunk_len = chunk.len();

            index = chunk[0] >> 2;
            encoded.push(self.list[index as usize]);

            if chunk_len == 1 {
                index = (chunk[0] & 0x3) << 4;
                encoded.push(self.list[index as usize]);
                encoded.push(PADDING);
                encoded.push(PADDING);
                continue;
            }

            index = ((chunk[0] & 0x3) << 4) | (chunk[1] >> 4);
            encoded.push(self.list[index as usize]);

            if chunk_len == 2 {
                index = (chunk[1] & 0xF) << 2;
                encoded.push(self.list[index as usize]);
                encoded.push(PADDING);
                continue;
            }

            index = ((chunk[1] & 0xF) << 2) | (chunk[2] >> 6);
            encoded.push(self.list[index as usize]);

            index = chunk[2] & 0x3F;
            encoded.push(self.list[index as usize]);
        }

        encoded
    }

    pub fn decode(&self, input: &str) -> Vec<u8> {
        let mut values = Vec::new();

        for c in input.chars() {
            if let Some(&v) = self.dict.get(&c) {
                values.push(v);
            } else if c == PADDING {
                values.push(u8::MAX);
            } else {
                panic!("Invalid Gua64 character: {}", c);
            }
        }

        let mut decoded = Vec::new();
        for chunk in values.chunks(4) {
            if chunk.len() < 4 {
                panic!("Invalid Gua64 encoding");
            }

            decoded.push((chunk[0] << 2) | (chunk[1] >> 4));

            if chunk[2] != u8::MAX {
                decoded.push((chunk[1] << 4) | (chunk[2] >> 2));
            }

            if chunk[3] != u8::MAX {
                decoded.push((chunk[2] << 6) | chunk[3]);
            }
        }

        decoded
    }

    pub fn verify(&self, input: &str) -> bool {
        input
            .chars()
            .all(|c| self.dict.contains_key(&c) || c == PADDING)
    }
}
