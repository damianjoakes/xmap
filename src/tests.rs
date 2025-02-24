use crate::maps::CIndexMap;

#[test]
fn test_insert() {
    let string_one = String::from("foo");
    let string_two = String::from("bar");
    let mut c_index: CIndexMap<String, String> = CIndexMap::new();

    c_index.insert(string_one.to_string(), string_two.to_string());
    c_index.insert(string_two.to_string(), string_one.to_string());

    dbg!(c_index.index(0));
    dbg!(c_index.index(1));
    c_index.remove(1).unwrap();
    dbg!(c_index.index(0).unwrap());
    dbg!(c_index.index(1));
}