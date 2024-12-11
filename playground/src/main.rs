#![forbid(unsafe_code)]

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    // Input
    let (input, set_input) = signal(String::new());
    // Output

    let output = view! {
        <table>
            <tbody>
            // # Implementation
            // - See the builder crate for a generator
            <tr>
            <td>"arabic_mathematical"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_arabic_mathematical(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"bold"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_bold(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"bold_italic"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_bold_italic(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"bold_fraktur"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_bold_fraktur(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"bold_script"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_bold_script(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"circled"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_circled(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"comma"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_comma(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"double_struck"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_double_struck(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"fraktur"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_fraktur(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"full_stop"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_full_stop(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"fullwidth"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_fullwidth(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"italic"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_italic(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"looped"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_looped(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"monospace"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_monospace(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"negative_circled"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_negative_circled(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"negative_squared"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_negative_squared(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"regional"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_regional(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"segmented"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_segmented(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"other"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_other(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"parenthesized"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_parenthesized(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"sans_serif_bold_italic"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_sans_serif_bold_italic(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"sans_serif_bold"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_sans_serif_bold(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"sans_serif_italic"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_sans_serif_italic(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"sans_serif"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_sans_serif(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"script"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_script(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"small_capital"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_small_capital(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"superscript"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_superscript(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"subscript"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_subscript(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"stretched"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_stretched(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"squared"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_squared(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"tailed"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_tailed(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>
            <tr>
            <td>"wide"</td>
            <td>{ move || -> String { input.get().chars()
            .map(|c| unicode_font::try_as_wide(&c).cloned().unwrap_or('�'))
            .collect() } }</td>
            </tr>

            </tbody>
        </table>
    };

    view! {
        <div>
            <h1>"Unicode font"</h1>
            <p>"Convert unicode characters between fonts using the Rust crate "<a href="https://github.com/saona-raimundo/unicode_font">"unicode_font"</a>"."</p>
        </div>
        <div>
            <h2>"Input"</h2>
            <textarea
                autofocus
                rows="3"
                cols="20"
                placeholder="Input to convert font."
                on:input=move |ev| {
                    set_input.set(event_target_value(&ev));
                }
                prop:value=input
            >
                {move || input.get()}
            </textarea>
        </div>
        <div>
            <h2>"Output"</h2>
            <div>{output}</div>
        </div>

        // Description
        <footer>
            <h2>"About"</h2>
            <p><a href="https://www.unicode.org/reports/tr44/tr44-32.html#Character_Decomposition_Mappings">"Unicode Standard Annex #44"</a>" defines Character Decomposition Mapping.
            In particular, characters are given a <font> tag to indicate some characters are a font variant of others.
            On top of these variants, we add carefully selected variants, like superscript, subscript and squared.
            This extension is included by default and can be turned off in the Rust crate."
            </p>
            <p>"Characters with font variants encoded in Unicode have a best-effort chosen plain variant. \
            Plain unicodes can be converted to different font variants using the unicode_font crate."</p>

            <h3>"This playground"</h3>
            <p>"When a (plain) insterted character does not have a font variant, we display �."</p>
            <p>
                "Use the input "
                <button on:click=move |_| set_input.set("123-abc-i-ABC-ابج-אדה".to_string())>"123-abc-i-ABC-ابج-אדה"</button>
                " to see a character converted to each font."
            </p>
            <p>
                "Source code: "
                <a href="https://github.com/saona-raimundo/unicode_font/playground">"GitHub"</a>
            </p>
            <p>
                "Lisence: "
                <a rel="lisence" href="https://creativecommons.org/publicdomain/zero/1.0/">
                    <img
                        alt="Creative Commons Licence"
                        style="border-width:0"
                        src="https://i.creativecommons.org/l/by/4.0/80x15.png"
                    />
                </a> <a rel="lisence" href="https://creativecommons.org/publicdomain/zero/1.0/">
                    {"CC0 1.0 Universal"}
                </a>
            </p>
            <address>
                {"Author: 🧑🏼‍💻"}
                <a href="href=https://saona-raimundo.github.io/">{"Raimundo Saona"}</a>
            </address>
        </footer>
    }
}
