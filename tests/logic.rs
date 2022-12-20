/*
 * Copyright (c) 2022 Janosch Reppnow <janoschre+rust@gmail.com>.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

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
