/*
 * Copyright (c) 2022 Janosch Reppnow <janoschre+rust@gmail.com>.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[test]
fn good() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/good/*.rs");
}
