/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[allow(warnings)]
mod bindings;

mod matcher;
mod theme;

use crate::bindings::servo::tailwind::console;
use bindings::exports::servo::tailwind::compiler::Guest;
use std::collections::HashSet;

struct Component;

impl Guest for Component {
    fn compile_classes(classes: String) -> String {
        let mut out = String::new();
        let mut seen = HashSet::new();

        for token in classes.split_whitespace() {
            if seen.insert(token) {
                if let Some(css) = matcher::match_utility(token) {
                    out.push_str(&matcher::serialize_class_rule(token, &css));
                }
            }
        }

        out
    }

    fn compile_candidates(candidates: Vec<String>) -> String {
        console::log(&format!(
            "railwind received {} candidates: {:?}",
            candidates.len(),
            candidates
        ));
        let mut out = String::new();
        let mut seen = HashSet::new();

        for token in candidates {
            if seen.insert(token.clone()) {
                if let Some(css) = matcher::match_utility(&token) {
                    out.push_str(&matcher::serialize_class_rule(&token, &css));
                }
            }
        }
        console::log(&format!(
            "railwind output {} length css: {:?}",
            out.len(),
            out
        ));
        out
    }
}

bindings::export!(Component with_types_in bindings);
