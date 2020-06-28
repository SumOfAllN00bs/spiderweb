    pub fn caesar(input: &str, shift: i32) -> String{
        let mut result: String = String::new();
        let trueshift: u32 = (((shift % 26) + 26) % 26) as u32;

        for c in input.chars() {
            let converted = c as u32;
            if !(converted >= 65 && converted <= 90) && !(converted >= 97 && converted <= 122) {
                result.push(std::char::from_u32(converted).unwrap());
                continue;
            }
            if converted >= 65 && converted <= 90 {
                result.push(std::char::from_u32((((converted - 65) + trueshift) % 26) + 65).unwrap());
            }
            if converted >= 97 && converted <= 122 {
                result.push(std::char::from_u32((((converted - 97) + trueshift) % 26) + 97).unwrap());
            }
        }

        result
    }