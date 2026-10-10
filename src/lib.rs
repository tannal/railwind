/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[allow(warnings)]
mod bindings;

mod matcher;
mod theme;

use crate::bindings::servo::tailwind::console;
// use crate::bindings::servo::dom::console;
use bindings::exports::servo::tailwind::compiler::Guest;
use std::collections::HashSet;

struct Component;

impl Guest for Component {
    fn compile_classes(classes: String) -> String {
        let candidates: Vec<String> = classes.split_whitespace().map(|s| s.to_string()).collect();
        Self::compile_candidates(candidates)
    }

    fn compile_candidates(candidates: Vec<String>) -> String {
        console::log(&format!(
            "railwind received {} candidates: {:?}\n",
            candidates.len(),
            candidates
        ));
        let mut seen = HashSet::new();
        let mut out = String::with_capacity(candidates.len() * 64);
        for candidate in &candidates {
            for token in candidate.split_whitespace() {
                if seen.insert(token) {
                    if let Some(css) = matcher::parse_candidate(token) {
                        out.push_str(&css);
                    }
                }
            }
        }
        console::log(&format!(
            "railwind output {} length css: {:?}\n",
            out.len(),
            out
        ));
        out
    }
}

bindings::export!(Component with_types_in bindings);
