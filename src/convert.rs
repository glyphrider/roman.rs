#[allow(dead_code)]

pub fn to(arabic: u16) -> String {
    let mut result = String::new();
    let mut working_arabic = arabic;
    let translate: Vec<(u16,&str)> = vec![(1,"I"),(4,"IV"),(5,"V"),(9,"IX"),(10,"X"),
    (40,"XL"),(50,"L"),(90,"XC"),(100,"C"),(400,"CD"),(500,"D"),(900,"CM"),(1000,"M")];

    for e in translate.iter().rev() {
        let (arabic_value,roman_value) = e;
        // pulling references out of the iterator means we have to deref arabic
        apply_translate_element_to_arabic_value(&mut working_arabic, &mut result, *arabic_value, roman_value);
    }
    return result
}

fn apply_translate_element_to_arabic_value(arabic: &mut u16, result: &mut String, arabic_value: u16, roman_value: &str) {
    // receiving arabic as a mut u16 ref means we have to deref it to use it as a u16 -- but we're accessing the original memory!
    while *arabic >= arabic_value {
        *arabic -= arabic_value;
        result.push_str(roman_value);
    }
}

pub fn from(roman: &str) -> u16 {
    let mut result: u16 = 0;
    let mut working_roman: String = String::from(roman);
    let translate: Vec<(u16,&str)> = vec![(1,"I"),(4,"IV"),(5,"V"),(9,"IX"),(10,"X"),
    (40,"XL"),(50,"L"),(90,"XC"),(100,"C"),(400,"CD"),(500,"D"),(900,"CM"),(1000,"M")];

    for e in translate.iter().rev() {
        let (arabic_value,roman_value) = e;
        apply_translate_element_to_roman_value(&mut working_roman, &mut result, *arabic_value, roman_value);
    }
    return result;
}

fn apply_translate_element_to_roman_value(roman: &mut String, result: &mut u16, arabic_value: u16, roman_value: &str) {
    while roman.starts_with(roman_value) {
        let working_roman: String = roman.clone();
        roman.clear();
        for (i,c) in working_roman.chars().enumerate() {
            if i >= roman_value.len() {
                roman.push(c);
            }
        }
        *result = *result + arabic_value;
    }
}