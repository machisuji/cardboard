/**
 * Creates a HashMap with owned Strings as keys and values.
 */
#[macro_export]
macro_rules! hash_from_string_to_string(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::linked_hash_map::LinkedHashMap::new();
            $(
                m.insert(String::from($key), String::from($value));
            )+
            m
        }
     };
);
