use crossroads::crossroads;

#[crossroads]
fn i() {
    match fork!() {
        am_legend => {}
        am_human => {}
    }
}

#[test]
fn actually_duplicates() {
    i_am_legend();
    i_am_human();
}

#[crossroads]
fn returns() -> usize {
    match fork!() {
        a_1 => 1,
        a_2 => 2,
    }
}

#[test]
fn with_value() {
    assert_eq!(1, returns_a_1());
    assert_eq!(2, returns_a_2());
}

// TODO: Up next!
// #[crossroads]
// fn recursively() -> usize {
//     match fork!() {
//         returns => {
//             match fork!() {
//                 a_1 => 1,
//                 a_3 => 3,
//             }
//         }
//         returns_a_2 => 2,
//     }
// }
