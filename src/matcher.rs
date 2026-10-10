/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::theme;
use std::fmt::Write;

/// Escapes CSS selector identifiers according to the CSSOM CSS.escape() specification.
pub fn escape_css_identifier(ident: &str) -> String {
    let mut escaped = String::with_capacity(ident.len() + 4);
    let mut chars = ident.chars().peekable();

    // Check if the identifier starts with a '-' followed by a digit or another '-'
    if let Some(&first) = chars.peek() {
        if first == '-' {
            escaped.push(chars.next().unwrap());
        }
    }

    for ch in chars {
        match ch {
            // Characters that must be escaped in CSS class selectors
            ':' | '/' | '.' | '[' | ']' | '!' | '#' | '%' | '(' | ')' | ',' | '@' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            // Standard valid identifier characters
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => {
                escaped.push(ch);
            }
            // Any other special character or whitespace
            _ => {
                escaped.push('\\');
                escaped.push(ch);
            }
        }
    }

    escaped
}

/// Parses an entire candidate token (with arbitrary variants) into standard CSS rules.
pub fn parse_candidate(candidate: &str) -> Option<String> {
    let mut parts: Vec<&str> = candidate.split(':').collect();
    let base_raw = parts.pop()?;

    let mut pseudo: Option<&'static str> = None;
    let mut media_query: Option<&'static str> = None;

    for variant in parts {
        match variant {
            "hover" => pseudo = Some(":hover"),
            "focus" => pseudo = Some(":focus"),
            "active" => pseudo = Some(":active"),
            "disabled" => pseudo = Some(":disabled"),
            "sm" => media_query = Some("(min-width: 640px)"),
            "md" => media_query = Some("(min-width: 768px)"),
            "lg" => media_query = Some("(min-width: 1024px)"),
            "xl" => media_query = Some("(min-width: 1280px)"),
            "dark" => media_query = Some("(prefers-color-scheme: dark)"),
            _ => return None,
        }
    }

    let (is_negative, base) = if let Some(stripped) = base_raw.strip_prefix('-') {
        (true, stripped)
    } else {
        (false, base_raw)
    };

    let declarations = match_base_utility(base, is_negative)?;

    let mut selector = String::new();
    selector.push('.');
    selector.push_str(&escape_css_identifier(candidate));

    if let Some(p) = pseudo {
        selector.push_str(p);
    }

    let mut rule = String::new();
    if let Some(mq) = media_query {
        let _ = write!(
            rule,
            "@media {} {{ {} {{ {} }} }}\n",
            mq, selector, declarations
        );
    } else {
        let _ = write!(rule, "{} {{ {} }}\n", selector, declarations);
    }

    Some(rule)
}

fn match_base_utility(class: &str, negative: bool) -> Option<String> {
    let neg = if negative { "-" } else { "" };

    // 1. Display & Visibility
    match class {
        "flex" => return Some("display: flex;".into()),
        "inline-flex" => return Some("display: inline-flex;".into()),
        "grid" => return Some("display: grid;".into()),
        "block" => return Some("display: block;".into()),
        "inline-block" => return Some("display: inline-block;".into()),
        "hidden" => return Some("display: none;".into()),
        "overflow-hidden" => return Some("overflow: hidden;".into()),
        "object-cover" => return Some("object-fit: cover;".into()),
        _ => {}
    }

    // 2. Alignment & Flexbox/Grid
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

    // 3. Sizing
    if class == "w-full" {
        return Some("width: 100%;".into());
    }
    if class == "h-full" {
        return Some("height: 100%;".into());
    }
    if class == "w-screen" {
        return Some("width: 100vw;".into());
    }
    if class == "h-screen" {
        return Some("height: 100vh;".into());
    }
    if class == "max-w-sm" {
        return Some("max-width: 24rem;".into());
    }
    if class == "max-w-md" {
        return Some("max-width: 28rem;".into());
    }
    if class == "max-w-lg" {
        return Some("max-width: 32rem;".into());
    }
    if class == "max-w-xl" {
        return Some("max-width: 36rem;".into());
    }

    if let Some(val) = class.strip_prefix("w-").and_then(theme::spacing) {
        return Some(format!("width: {val};"));
    }
    if let Some(val) = class.strip_prefix("h-").and_then(theme::spacing) {
        return Some(format!("height: {val};"));
    }

    // 4. Spacing (Paddings & Margins)
    if let Some(val) = class.strip_prefix("p-").and_then(theme::spacing) {
        return Some(format!("padding: {val};"));
    }
    if let Some(val) = class.strip_prefix("px-").and_then(theme::spacing) {
        return Some(format!("padding-left: {val}; padding-right: {val};"));
    }
    if let Some(val) = class.strip_prefix("py-").and_then(theme::spacing) {
        return Some(format!("padding-top: {val}; padding-bottom: {val};"));
    }
    if let Some(val) = class.strip_prefix("pt-").and_then(theme::spacing) {
        return Some(format!("padding-top: {val};"));
    }
    if let Some(val) = class.strip_prefix("pb-").and_then(theme::spacing) {
        return Some(format!("padding-bottom: {val};"));
    }
    if let Some(val) = class.strip_prefix("pl-").and_then(theme::spacing) {
        return Some(format!("padding-left: {val};"));
    }
    if let Some(val) = class.strip_prefix("pr-").and_then(theme::spacing) {
        return Some(format!("padding-right: {val};"));
    }

    if let Some(val) = class.strip_prefix("m-").and_then(theme::spacing) {
        return Some(format!("margin: {neg}{val};"));
    }
    if let Some(val) = class.strip_prefix("mx-").and_then(theme::spacing) {
        return Some(format!(
            "margin-left: {neg}{val}; margin-right: {neg}{val};"
        ));
    }
    if let Some(val) = class.strip_prefix("my-").and_then(theme::spacing) {
        return Some(format!(
            "margin-top: {neg}{val}; margin-bottom: {neg}{val};"
        ));
    }
    if let Some(val) = class.strip_prefix("mt-").and_then(theme::spacing) {
        return Some(format!("margin-top: {neg}{val};"));
    }
    if let Some(val) = class.strip_prefix("mb-").and_then(theme::spacing) {
        return Some(format!("margin-bottom: {neg}{val};"));
    }

    // 5. Gap & Sibling Spacing
    if let Some(val) = class.strip_prefix("gap-").and_then(theme::spacing) {
        return Some(format!("gap: {val};"));
    }
    if let Some(val) = class.strip_prefix("space-x-").and_then(theme::spacing) {
        return Some(format!("column-gap: {val};"));
    }

    // 6. Colors & Typography
    if let Some(val) = class.strip_prefix("bg-").and_then(theme::color) {
        return Some(format!("background-color: {val};"));
    }
    if let Some(val) = class.strip_prefix("text-").and_then(theme::color) {
        return Some(format!("color: {val};"));
    }
    if let Some((size, line_height)) = class.strip_prefix("text-").and_then(theme::font_size) {
        return Some(format!("font-size: {size}; line-height: {line_height};"));
    }

    match class {
        "font-bold" => return Some("font-weight: 700;".into()),
        "font-semibold" => return Some("font-weight: 600;".into()),
        "font-medium" => return Some("font-weight: 500;".into()),
        "leading-snug" => return Some("line-height: 1.375;".into()),
        "leading-none" => return Some("line-height: 1;".into()),
        "leading-tight" => return Some("line-height: 1.25;".into()),
        _ => {}
    }

    // 7. Borders
    if class == "border" {
        return Some("border-width: 1px; border-style: solid;".into());
    }
    if class == "border-t" {
        return Some("border-top-width: 1px; border-top-style: solid;".into());
    }
    if class == "border-b" {
        return Some("border-bottom-width: 1px; border-bottom-style: solid;".into());
    }
    if let Some(val) = class.strip_prefix("border-").and_then(theme::color) {
        return Some(format!("border-color: {val};"));
    }

    // 8. Radii
    if class == "rounded" {
        return Some("border-radius: 0.25rem;".into());
    }
    if class == "rounded-full" {
        return Some("border-radius: 9999px;".into());
    }
    if let Some(val) = class
        .strip_prefix("rounded-")
        .and_then(theme::border_radius)
    {
        return Some(format!("border-radius: {val};"));
    }

    // 9. Box Shadows
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
        "shadow-xl" => {
            return Some(
                "box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);"
                    .into(),
            );
        }
        _ => {}
    }

    // 10. Transforms & Transitions
    if class == "transition-all" {
        return Some("transition-property: all; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".into());
    }
    if class == "transition-colors" {
        return Some("transition-property: color, background-color, border-color; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".into());
    }
    if class == "duration-300" {
        return Some("transition-duration: 300ms;".into());
    }
    if let Some(val) = class.strip_prefix("translate-y-").and_then(theme::spacing) {
        return Some(format!("transform: translateY({neg}{val});"));
    }

    // 11. Multi-line Clamping
    if class == "line-clamp-3" {
        return Some("overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 3;".into());
    }

    None
}
