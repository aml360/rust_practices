#[derive(Clone, Copy)]
struct U8Iter {
    byte: u8,
    indx: u8,
}

fn main() {
    let byte: U8Iter = 0x4u8.into();
    let xd = Into::<u8>::into(byte);
    println!("{:b}", xd);
    for bit in byte.into_iter().rev() {
        let bit = bit & 0x1;
        print!("{}", bit)
    }
    print!("\n")
}

impl Iterator for U8Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.indx {
            0 | 0..=7 => {
                let next_item = Some(self.byte >> self.indx);
                self.indx += 1;
                next_item
            }
            _ => None,
        }
    }
}

impl DoubleEndedIterator for U8Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.indx {
            0 | 0..=7 => {
                let next_item = Some(self.byte >> (7 - self.indx));
                self.indx += 1;
                next_item
            }
            _ => None,
        }
    }
}

impl From<u8> for U8Iter {
    fn from(v: u8) -> Self {
        Self { byte: v, indx: 0 }
    }
}

impl Into<u8> for U8Iter {
    fn into(self) -> u8 {
        self.byte
    }
}
