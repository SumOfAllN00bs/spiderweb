fn main() {
    // println!("{}", caesar("test phrase", 10));
    // println!("{}", caesar("TEST PHRASE", 10));
    // println!("{}", ((-100 % 26) + 26) % 26);
}

fn caesar(_input: &str, _shift: i32) -> String{
    let mut _result: String = String::new();
    let mut _trueshift: u32 = (((_shift % 26) + 26) % 26) as u32;

    for c in _input.chars() {
        let converted = c as u32;
        if !(converted >= 65 && converted <= 90) && !(converted >= 97 && converted <= 122) {
            _result.push(std::char::from_u32(converted).unwrap());
            continue;
        }
        if converted >= 65 && converted <= 90 {
            _result.push(std::char::from_u32((((converted - 65) + _trueshift) % 26) + 65).unwrap());
        }
        if converted >= 97 && converted <= 122 {
            _result.push(std::char::from_u32((((converted - 97) + _trueshift) % 26) + 97).unwrap());
        }
    }

    _result
}

#[cfg(test)]
mod tests;