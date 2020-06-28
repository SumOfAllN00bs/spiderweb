fn main() {
    //caesar("test phrase", 10);
    caesar("TEST PHRASE", 10);
    //println!("{}", ((-100 % 26) + 26) % 26);
}

fn caesar(_input: &str, _shift: i32) -> String{
    let _upper: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let _lower: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut _result: String = String::new();
    let trueshift = ((_shift % 26) + 26) % 26;

    for c in _input.chars() {
        let converted = c as u32;
        if converted <= 64 || converted >= 123 {
            _result.push(std::char::from_u32(converted).unwrap());
            continue;
        }
        if converted >= 65 && converted <= 90 {

        }
    }

    _result
}

#[cfg(test)]
mod tests;