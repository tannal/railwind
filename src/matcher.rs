/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::theme;
use cssparser::ToCss;
use std::fmt::Write;

pub fn match_utility(class: &str) -> Option<String> {
    // 1. Layout & Display
    match class {
        "flex" => return Some("display: flex;".into()),
        "inline-flex" => return Some("display: inline-flex;".into()),
        "grid" => return Some("display: grid;".into()),
        "block" => return Some("display: block;".into()),
        "inline-block" => return Some("display: inline-block;".into()),
        "hidden" => return Some("display: none;".into()),
        _ => {}
    }

    // 2. Alignment
    match class {
        "items-center" => return Some("align-items: center;".into()),
        "items-start" => return Some("align-items: flex-start;".into()),
        "items-end" => return Some("align-items: flex-end;".into()),
        "justify-between" => return Some("justify-content: space-between;".into()),
        "justify-center" => return Some("justify-content: center;".into()),
        "justify-start" => return Some("justify-content: flex-start;".into()),
        "justify-end" => return Some("justify-content: flex-end;".into()),
        _ => {}
    }

    // 3. Spacing
    if let Some(val) = class.strip_prefix("p-").and_then(theme::spacing) {
        return Some(format!("padding: {val};"));
    }
    if let Some(val) = class.strip_prefix("px-").and_then(theme::spacing) {
        return Some(format!("padding-left: {val}; padding-right: {val};"));
    }
    if let Some(val) = class.strip_prefix("py-").and_then(theme::spacing) {
        return Some(format!("padding-top: {val}; padding-bottom: {val};"));
    }
    if let Some(val) = class.strip_prefix("m-").and_then(theme::spacing) {
        return Some(format!("margin: {val};"));
    }
    if let Some(val) = class.strip_prefix("mx-").and_then(theme::spacing) {
        return Some(format!("margin-left: {val}; margin-right: {val};"));
    }
    if let Some(val) = class.strip_prefix("my-").and_then(theme::spacing) {
        return Some(format!("margin-top: {val}; margin-bottom: {val};"));
    }

    // 4. Background & Text Colors
    if let Some(val) = class.strip_prefix("bg-").and_then(theme::color) {
        return Some(format!("background-color: {val};"));
    }
    if let Some(val) = class.strip_prefix("text-").and_then(theme::color) {
        return Some(format!("color: {val};"));
    }

    // 5. Typography
    if let Some((size, line_height)) = class.strip_prefix("text-").and_then(theme::font_size) {
        return Some(format!("font-size: {size}; line-height: {line_height};"));
    }
    match class {
        "font-bold" => return Some("font-weight: 700;".into()),
        "font-semibold" => return Some("font-weight: 600;".into()),
        "font-medium" => return Some("font-weight: 500;".into()),
        _ => {}
    }

    // 6. Border Radius
    if class == "rounded" {
        return Some("border-radius: 0.25rem;".into());
    }
    if let Some(val) = class
        .strip_prefix("rounded-")
        .and_then(theme::border_radius)
    {
        return Some(format!("border-radius: {val};"));
    }

    // 7. Box Shadow
    match class {
        "shadow-sm" => return Some("box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);".into()),
        "shadow" => {
            return Some(
                "box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);".into(),
            );
        }
        "shadow-lg" => {
            return Some(
                "box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);"
                    .into(),
            );
        }
        _ => {}
    }

    None
}

/// Serializes a class name safely using cssparser rules
pub fn serialize_class_rule(class: &str, declarations: &str) -> String {
    let mut out = String::new();
    out.push('.');
    // Escape identifiers containing dots, colons, or special characters
    cssparser::serialize_identifier(class, &mut out).unwrap();
    write!(out, " {{ {} }}\n", declarations).unwrap();
    out
}
