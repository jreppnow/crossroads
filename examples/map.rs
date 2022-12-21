#[crossroads::crossroads]
#[test]
fn empty() {
    use std::collections::HashMap;

    let mut map = HashMap::<String, usize>::default();

    match fork!() {
        by_default => {}
        after_add => {
            map.insert("Key".to_owned(), 1337);
            match fork!() {
                and_remove => map.remove("Key"),
                and_clear => map.clear(),
            };
        }
    }

    assert!(map.is_empty());
}
