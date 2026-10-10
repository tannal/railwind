/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub fn spacing(key: &str) -> Option<&'static str> {
    match key {
        "0" => Some("0px"),
        "px" => Some("1px"),
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
        "7" => Some("1.75rem"),
        "8" => Some("2rem"),
        "9" => Some("2.25rem"),
        "10" => Some("2.5rem"),
        "11" => Some("2.75rem"),
        "12" => Some("3rem"),
        "14" => Some("3.5rem"),
        "16" => Some("4rem"),
        "20" => Some("5rem"),
        "24" => Some("6rem"),
        "28" => Some("7rem"),
        "32" => Some("8rem"),
        "36" => Some("9rem"),
        "40" => Some("10rem"),
        "44" => Some("11rem"),
        "48" => Some("12rem"),
        "52" => Some("13rem"),
        "56" => Some("14rem"),
        "60" => Some("15rem"),
        "64" => Some("16rem"),
        "72" => Some("18rem"),
        "80" => Some("20rem"),
        "96" => Some("24rem"),
        "auto" => Some("auto"),
        "full" => Some("100%"),
        "screen" => Some("100vw"),
        _ => None,
    }
}

pub fn color(key: &str) -> Option<&'static str> {
    match key {
        "white" => Some("#ffffff"),
        "black" => Some("#000000"),
        "transparent" => Some("transparent"),
        "current" => Some("currentColor"),

        // Gray
        "gray-50" => Some("#f9fafb"),
        "gray-100" => Some("#f3f4f6"),
        "gray-200" => Some("#e5e7eb"),
        "gray-300" => Some("#d1d5db"),
        "gray-400" => Some("#9ca3af"),
        "gray-500" => Some("#6b7280"),
        "gray-600" => Some("#4b5563"),
        "gray-700" => Some("#374151"),
        "gray-800" => Some("#1f2937"),
        "gray-900" => Some("#111827"),

        // Slate
        "slate-50" => Some("#f8fafc"),
        "slate-100" => Some("#f1f5f9"),
        "slate-200" => Some("#e2e8f0"),
        "slate-800" => Some("#1e293b"),
        "slate-900" => Some("#0f172a"),

        // Indigo
        "indigo-50" => Some("#eef2ff"),
        "indigo-100" => Some("#e0e7ff"),
        "indigo-200" => Some("#c7d2fe"),
        "indigo-500" => Some("#6366f1"),
        "indigo-600" => Some("#4f46e5"),
        "indigo-700" => Some("#4338ca"),
        "indigo-800" => Some("#3730a3"),
        "indigo-900" => Some("#312e81"),

        // Blue
        "blue-500" => Some("#3b82f6"),
        "blue-600" => Some("#2563eb"),
        "blue-700" => Some("#1d4ed8"),

        // Red & Green
        "red-500" => Some("#ef4444"),
        "red-600" => Some("#dc2626"),
        "green-500" => Some("#22c55e"),
        "green-600" => Some("#16a34a"),

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
        "3xl" => Some(("1.875rem", "2.25rem")),
        _ => None,
    }
}

pub fn border_radius(key: &str) -> Option<&'static str> {
    match key {
        "none" => Some("0px"),
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
