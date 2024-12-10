const FONTS: [&str; 32] = [
    "arabic_mathematical",
    "bold",
    "bold_italic",
    "bold_fraktur",
    "bold_script",
    "circled",
    "comma",
    "double_struck",
    "fraktur",
    "full_stop",
    "fullwidth",
    "italic",
    "looped",
    "monospace",
    "negative_circled",
    "negative_squared",
    "regional",
    "segmented",
    "other",
    "parenthesized",
    "sans_serif_bold_italic",
    "sans_serif_bold",
    "sans_serif_italic",
    "sans_serif",
    "script",
    "small_capital",
    "superscript",
    "subscript",
    "stretched",
    "squared",
    "tailed",
    "wide",
];

fn main() {
    for font in FONTS {
        println!(
            "\
            <tr>\n\
                <td>\"{font}\"</td>\n\
                <td>{{ move || -> String {{ input.get().chars()\n\
                        .map(|c| unicode_font::try_as_{font}(&c).cloned().unwrap_or('�'))\n\
                        .collect() }} }}</td>\n\
            </tr>\
        "
        );
    }
}
