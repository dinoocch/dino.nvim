/* Based on nvim-colorizer.lua */
use colors_transform::{Color, Rgb};
use lazy_static::lazy_static;
use nvim_oxi::types::AutocmdCallbackArgs;
use nvim_oxi::{self as oxi, api, api::Buffer, opts::BufAttachOpts, opts::*};
use regex::Regex;

lazy_static! {
    static ref HEX_COLOR_RE: Regex = Regex::new(r"#[A-Fa-f0-9]{6}").unwrap();
}

// Based on color_is_bright from nvim-colorizer.lua
fn fg_color(color: &str) -> &str {
    let rgb = Rgb::from_hex_str(color).unwrap_or(Rgb::from(100.0, 100.0, 100.0));
    if rgb.get_lightness() >= 40.0 {
        "Black"
    } else {
        "White"
    }
}

fn highlight_range(
    mut buf: Buffer,
    ns: u32,
    first_line: usize,
    last_line: usize,
) -> oxi::Result<()> {
    let lines = buf.get_lines(first_line, last_line, false)?;
    buf.clear_namespace(ns, first_line, last_line)?;
    for (line_index, line) in lines.enumerate() {
        for color_match in HEX_COLOR_RE.find_iter(line.as_str().unwrap()) {
            let mut hl_name = color_match.as_str().to_ascii_uppercase();
            hl_name.remove(0);
            let hl_name = format!("COLOR_{}", hl_name);
            // Probably a hashmap cache would be faster than this?
            match api::get_hl_by_name(&hl_name, false) {
                Ok(_) => {}
                Err(_) => {
                    api::set_hl(
                        0,
                        &hl_name,
                        Some(
                            &SetHighlightOpts::builder()
                                .foreground(fg_color(color_match.as_str()))
                                .background(color_match.as_str())
                                .build(),
                        ),
                    )?;
                }
            }
            buf.add_highlight(
                ns,
                &hl_name,
                (line_index + first_line) as i64,
                color_match.start() as i64,
                color_match.end() as i64,
            )?;
        }
    }
    Ok(())
}

fn highlight_buffer(buf: Buffer, ns: u32) -> oxi::Result<()> {
    highlight_range(buf, ns, 0, 50000)
}

fn on_lines(
    ns: u32,
    (_, buf, _, first_line, _, last_line, _, _, _): (
        String,
        Buffer,
        u32,
        usize,
        usize,
        usize,
        usize,
        Option<usize>,
        Option<usize>,
    ),
) -> oxi::Result<bool> {
    highlight_range(buf, ns, first_line, last_line)?;
    // true implies detch. therefore always false
    Ok(false)
}

pub fn colorize_buffer(args: AutocmdCallbackArgs) -> oxi::Result<bool> {
    let ns = api::create_namespace("colorizer");
    args.buffer.attach(
        true,
        &BufAttachOpts::builder()
            .on_lines(move |x| on_lines(ns, x))
            .preview(true)
            .build(),
    )?;
    highlight_buffer(args.buffer, ns)?;

    Ok(false)
}

pub fn setup_colorizer() -> oxi::Result<()> {
    let colorizer_group = api::create_augroup("colorize", None)?;

    api::create_autocmd(
        vec!["BufRead", "BufNewFile", "FileType"],
        &CreateAutocmdOpts::builder()
            .group(colorizer_group)
            .callback(colorize_buffer)
            .build(),
    )?;

    Ok(())
}
