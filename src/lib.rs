use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[wasm_bindgen]
pub fn string_to_binary(msg: String) -> String {
    msg.to_string();
    let mut msg_in_binary = "".to_string();
    for character in msg.clone().into_bytes() {
        msg_in_binary += &format!("0{:b} ", character)
    }
    msg_in_binary
}

#[wasm_bindgen]
pub fn binary_to_string(msg: &str) -> String {
    return msg
        .split(" ")
        .map(|n| u32::from_str_radix(n, 2).unwrap())
        .map(|c| char::from_u32(c).unwrap())
        .collect();
}

#[cfg(test)]
mod test {
    use crate::add;
    use crate::binary_to_string;
    use crate::string_to_binary;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_add_function() {
        assert_eq!(add(1, 1), 2);
        assert_eq!(add(10, 1), 11);
        assert_eq!(add(100, 1), 101);
    }

    #[wasm_bindgen_test]
    fn test_string_to_binary_function() {
        assert_eq!(string_to_binary("a".to_string()), "01100001 ".to_string());
        assert_eq!(
            string_to_binary("ab".to_string()),
            "01100001 01100010 ".to_string()
        );
        assert_eq!(
            string_to_binary("klos71".to_string()),
            "01101011 01101100 01101111 01110011 0110111 0110001 ".to_string()
        );
    }

    #[wasm_bindgen_test]
    fn test_binary_to_string() {
        assert_eq!(binary_to_string("01100001"), "a");
        assert_eq!(binary_to_string("01100001 01100010"), "ab".to_string());
        assert_eq!(
            binary_to_string("01101011 01101100 01101111 01110011 0110111 0110001"),
            "klos71".to_string()
        );
    }
}
