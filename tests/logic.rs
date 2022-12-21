/*
 * Copyright (c) 2022 Janosch Reppnow <janoschre+rust@gmail.com>.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
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

#[test]
fn preference() {
    #[crossroads]
    fn preference() -> usize {
        // Note: For some reason, match x { .. } + match y  { .. } as the return line does not parse
        // in Rust in general, not just with our macro..
        #[allow(clippy::needless_return)]
        return match fork!() {
            does => 1 + 3,
        } * match fork!() {
            matter => 2,
        };
    }

    assert_eq!(8, preference_does_matter());
}
