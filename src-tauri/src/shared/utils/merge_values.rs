use serde_json::{Value};

pub fn merge_values(curr: &mut Value, new: &Value) {
    match (curr, new) {
        (Value::Object(curr_map), Value::Object(new_map)) => {
            for (k, v) in new_map {
                match curr_map.get_mut(k) {
                    Some(existing) => merge_values(existing, v),
                    None => { curr_map.insert(k.clone(), v.clone()); }
                }
            }
        }
        (curr_slot, new_value) => {
            *curr_slot = new_value.clone();
        }
    }
}
