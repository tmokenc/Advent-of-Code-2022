use std::str::FromStr;

pub fn lines_to_vec<T: FromStr>(s: &str) -> Option<Vec<T>> {
    let mut list = Vec::new();

    for data in s.lines() {
        let value = data.parse::<T>().ok()?;
        list.push(value);
    }

    Some(list)
}

pub fn string_to_vec<T: FromStr>(s: &str, seperator: &str) -> Option<Vec<T>> {
    let mut list = Vec::new();

    for data in s.split(seperator) {
        let value = data.parse::<T>().ok()?;
        list.push(value);
    }

    Some(list)
}
