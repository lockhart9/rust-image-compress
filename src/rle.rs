pub trait Encode {
    fn encode(&self, data: &[u8]) -> Vec<u8>;
}

pub struct SimpleRle;
impl SimpleRle {
    pub fn new() -> Self {
        SimpleRle {}
    }

    fn get_run_length(
        &self,
        data: &[u8],
        x: usize,
        limit: usize,
        run: &mut u8,
        code: &mut u8,
    ) -> usize {
        let mut offset = x;
        *run = 0;
        *code = data[offset];
        while offset < data.len() && *code == data[offset] && (*run as usize) < limit {
            offset += 1;
            *run += 1;
        }
        offset
    }
}

impl Encode for SimpleRle {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        let mut run: u8 = 0;
        let mut code: u8 = 0;
        let mut offset = 0;
        let mut out = vec![];
        if data.len() == 0 {
            return out;
        }

        while offset < data.len() {
            offset = self.get_run_length(data, offset, 0xff, &mut run, &mut code);
            out.push(run);
            out.push(code);
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_runlength_1() {
        let encoded = SimpleRle::new().encode("aaabbc".as_bytes());
        assert_eq!(Some(&3), encoded.get(0));
        assert_eq!(Some(&b'a'), encoded.get(1));
        assert_eq!(Some(&2), encoded.get(2));
        assert_eq!(Some(&b'b'), encoded.get(3));
        assert_eq!(Some(&1), encoded.get(4));
        assert_eq!(Some(&b'c'), encoded.get(5));
        assert_eq!(None, encoded.get(6));
    }
}
