use nvim_oxi as oxi;

mod colorscheme;
mod keybindings;
mod options;


#[oxi::module]
fn dino() -> oxi::Result<()> {
    colorscheme::setup_colors()?;
    keybindings::setup_keymaps()?;
    options::setup_options()?;

    Ok(())
}
