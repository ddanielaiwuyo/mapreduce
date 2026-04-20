pub mod worker;

// For the tests to have access them. Later we can deicpher fn pointer vs fn item
pub fn custom_map(_key: &String, value: &str) -> Vec<worker::MKeyValue> {
    let mut cleaned_content: String = String::with_capacity(value.len());

    for ch in value.chars() {
        if ch.is_whitespace() || ch.is_alphabetic() {
            let us = format!("{}", ch);
            cleaned_content += &us;
        }
    }

    let value = cleaned_content.to_ascii_lowercase();

    let words = value.split_ascii_whitespace();
    let mut kv_pairs = vec![];
    for word in words {
        kv_pairs.push(worker::MKeyValue {
            key: String::from(word),
            value: String::from("1"),
        });
    }

    kv_pairs
}

pub fn custom_reduce(key: String, values: Vec<String>) -> String {
    let mut curr_count: u32 = 0;
    for val in values {
        let s = val.parse::<u32>().unwrap_or_default();
        curr_count += s;
    }

    // Matching the format provided by the exercise
    format!("{}: {}\n", key, curr_count)
}
