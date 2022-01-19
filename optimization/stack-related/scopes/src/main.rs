#[derive(Debug)]
pub struct BigStruct {
    pub num: i64,
    pub num2: i64,
    pub num3: i64,
    pub num4: i64,
    pub num5: i64,
    pub num6: i64,
    pub num7: i64,
    pub num8: i64,
    pub num9: i64,
    pub num10: i64,
    pub num11: i64,
    pub num12: i64,
    pub num13: i64,
    pub num14: i64,
    pub num15: i64,
    pub num16: i64,
    pub num17: i64,
    pub num18: i64,
    pub num19: i64,
    pub num20: i64,
    pub num21: i64,
    pub num22: i64,
    pub num23: i64,
    pub num24: i64,
    pub num25: i64,
    pub num26: i64,
    pub num27: i64,
    pub num29: i64,
    pub num30: i64,
}

impl BigStruct {
    pub fn new() -> BigStruct {
        BigStruct {
            num: 0,
            num2: 0,
            num3: 0,
            num4: 0,
            num5: 0,
            num6: 0,
            num7: 0,
            num8: 0,
            num9: 0,
            num10: 0,
            num11: 0,
            num12: 0,
            num13: 0,
            num14: 0,
            num15: 0,
            num16: 0,
            num17: 0,
            num18: 0,
            num19: 0,
            num20: 0,
            num21: 0,
            num22: 0,
            num23: 0,
            num24: 0,
            num25: 0,
            num26: 0,
            num27: 50,
            num29: 50,
            num30: 50,
        }
    }
}

pub fn main() {
    println!("Size fo BigStruct: {}", std::mem::size_of::<BigStruct>());

    let result = {
        let big_struct1 = BigStruct::new();
        println!("Obtained num: {:?}", big_struct1);
        big_struct1.num25
    };

    let result2 = {
        let big_struct2 = BigStruct::new();
        println!("Obtained num: {:?}", big_struct2);
        big_struct2.num25
    };

    let result3 = {
        let big_struct3 = BigStruct::new();
        println!("Obtained num: {:?}", big_struct3);
        big_struct3.num25
    };
    let result4 = {
        let big_struct4 = BigStruct::new();
        println!("Obtained num: {:?}", big_struct4);
        big_struct4.num25
    };

    println!("Obtained num: {}", result);
    println!("Obtained num: {}", result2);
    println!("Obtained num: {}", result3);
    println!("Obtained num: {}", result4);
}
