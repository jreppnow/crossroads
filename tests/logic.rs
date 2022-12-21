/*
 * Copyright (c) 2022 Janosch Reppnow <janoschre+rust@gmail.com>.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crossroads::crossroads;

#[test]
fn actually_duplicates() {
    #[crossroads]
    fn i() {
        match fork!() {
            am_legend => {}
            am_human => {}
        }
    }

    i_am_legend();
    i_am_human();
}

#[test]
fn with_value() {
    #[crossroads]
    fn returns() -> usize {
        match fork!() {
            a_1 => 1,
            a_2 => 2,
        }
    }

    assert_eq!(1, returns_a_1());
    assert_eq!(2, returns_a_2());
}

#[test]
fn recursive() {
    #[crossroads]
    fn recursively() -> usize {
        match fork!() {
            returns => match fork!() {
                a_1 => 1,
                a_3 => 3,
            },
            returns_a_2 => 2,
        }
    }

    assert_eq!(1, recursively_returns_a_1());
    assert_eq!(2, recursively_returns_a_2());
    assert_eq!(3, recursively_returns_a_3());
}

#[test]
fn leaves_function_without_forks_intact() {
    #[crossroads]
    fn does_not_actually_need_crossroads() {}

    does_not_actually_need_crossroads();
}

#[test]
fn single_path() {
    #[crossroads]
    fn add() -> usize {
        // Note: For some reason, match x { .. } + match y  { .. } as the return line does not parse
        // in Rust in general, not just with our macro..
        #[allow(clippy::needless_return)]
        return match fork!() {
            a_1 => 1,
        } + match fork!() {
            and_a_2 => 2,
        };
    }

    assert_eq!(3, add_a_1_and_a_2());
}
