pub enum IsDigit {
    Yes(i32),
    No,
}

pub struct AsDigitsIter<'a>(&'a [u8]);

impl<'a> Iterator for AsDigitsIter<'a> {
    type Item = IsDigit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() == 0 {
            return None;
        }

        let (head, tail) = self.0.split_at(1);
        self.0 = tail;

        match head[0] {
            b'0'...b'9' => Some(IsDigit::Yes((head[0] - b'0') as _)),
            _ => Some(IsDigit::No),
        }
    }
}

pub fn as_digits<'a>(chars: &'a str) -> AsDigitsIter<'a> {
    AsDigitsIter(chars.as_bytes())
}

#[cfg(test)]
mod convert_to_digits_should {
    use super::*;

    #[test]
    fn produce_as_stream_of_digits() {
        const NUMBERS: &'static str = "123456";

        assert_eq!(
            vec![1,2,3,4,5,6],
            as_digits(NUMBERS)
                .filter_map(|n| match n {
                    IsDigit::Yes(v) => Some(v),
                    IsDigit::No => None
                })
                .collect::<Vec<_>>());
    }
}
