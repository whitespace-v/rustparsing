use serde_json::Value;
use std::collections::HashMap;

pub fn merge(map1: &mut HashMap<String, String>, map2: HashMap<String, String>) {
    map1.extend(map2);
}
