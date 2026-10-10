/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub fn spacing(key: &str) -> Option<&'static str> {
    match key {
        "0" => Some("0px"),
        "0.5" => Some("0.125rem"),
        "1" => Some("0.25rem"),
        "1.5" => Some("0.375rem"),
        "2" => Some("0.5rem"),
        "2.5" => Some("0.625rem"),
        "3" => Some("0.75rem"),
        "3.5" => Some("0.875rem"),
        "4" => Some("1rem"),
        "5" => Some("1.25rem"),
        "6" => Some("1.5rem"),
        "8" => Some("2rem"),
        "10" => Some("2.5rem"),
        "12" => Some("3rem"),
        "16" => Some("4rem"),
        "20" => Some("5rem"),
        "24" => Some("6rem"),
        "32" => Some("8rem"),
        "full" => Some("100%"),
        "screen" => Some("100vh"),
        _ => None,
    }
}

pub fn color(name: &str) -> Option<&'static str> {
    match name {
        "white" => Some("#ffffff"),
        "black" => Some("#000000"),
        "transparent" => Some("transparent"),
        // Slate scale
        "slate-50" => Some("#f8fafc"),
        "slate-100" => Some("#f1f5f9"),
        "slate-200" => Some("#e2e8f0"),
        "slate-300" => Some("#cbd5e1"),
        "slate-400" => Some("#94a3b8"),
        "slate-500" => Some("#64748b"),
        "slate-600" => Some("#475569"),
        "slate-700" => Some("#334155"),
        "slate-800" => Some("#1e293b"),
        "slate-900" => Some("#0f172a"),
        "slate-950" => Some("#020617"),
        // Blue scale
        "blue-500" => Some("#3b82f6"),
        "blue-600" => Some("#2563eb"),
        // Red scale
        "red-500" => Some("#ef4444"),
        // Emerald scale
        "emerald-500" => Some("#10b981"),
        _ => None,
    }
}

pub fn border_radius(key: &str) -> Option<&'static str> {
    match key {
        "" => Some("0.25rem"),
        "sm" => Some("0.125rem"),
        "md" => Some("0.375rem"),
        "lg" => Some("0.5rem"),
        "xl" => Some("0.75rem"),
        "2xl" => Some("1rem"),
        "3xl" => Some("1.5rem"),
        "full" => Some("9999px"),
        _ => None,
    }
}

pub fn font_size(key: &str) -> Option<(&'static str, &'static str)> {
    match key {
        "xs" => Some(("0.75rem", "1rem")),
        "sm" => Some(("0.875rem", "1.25rem")),
        "base" => Some(("1rem", "1.5rem")),
        "lg" => Some(("1.125rem", "1.75rem")),
        "xl" => Some(("1.25rem", "1.75rem")),
        "2xl" => Some(("1.5rem", "2rem")),
        _ => None,
    }
}
