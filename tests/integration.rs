use x_map::maps::CIndexMap;

#[test]
pub fn test_integration() {
    let string_one = String::from("foo");
    let string_two = String::from("bar");

    let mut map = CIndexMap::new();
    map.insert(string_one.to_string(), string_two.to_string()).unwrap();

    dbg!(map.get(string_one.to_string()));
    dbg!(map.get_no_peq(string_one.to_string()));
}

#[test]
pub fn test_integration_2() {
    let mut map = CIndexMap::new();
    map.insert("foo", "bar").unwrap();

    dbg!(map.get("foo").unwrap());
    dbg!(map.get_no_peq("foo").unwrap());
}

#[test]
pub fn test_integration_3() {
    let mut map = CIndexMap::new();
    map.insert("foo", "bar").unwrap();
    dbg!(map.contains_key("foo")); // true
    dbg!(map.contains_key("bar")); // false
    dbg!(map.contains_value("bar")); // true
    dbg!(map.contains_value("foo")); // false
}