use nvim_oxi as oxi;

mod colorizer;
mod colorscheme;
mod keybindings;
mod options;
mod packages;
// mod completion;

#[oxi::module]
fn dino() -> oxi::Result<()> {
    colorscheme::setup_colors()?;
    keybindings::setup_keymaps()?;
    options::setup_options()?;
    packages::setup_packages()?;
    colorizer::setup_colorizer()?;

    // completion::setup_completion()?;

    Ok(())
}

