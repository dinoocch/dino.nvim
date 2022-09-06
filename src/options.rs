use std::fs;
use std::path::{Path, PathBuf};

use nvim_oxi::{self as oxi, api};

pub fn setup_options() -> oxi::Result<()> {
    api::set_option_value("colorcolumn", "80", None)?;
    api::set_option_value("cursorline", true, None)?;
    api::set_option_value("scrolloff", 4, None)?;
    api::set_option_value("splitbelow", true, None)?;
    api::set_option_value("splitright", true, None)?;
    api::set_option_value("wrap", false, None)?;

    // Disable conceal
    api::set_option_value("conceallevel", 0, None)?;
    api::set_option_value("concealcursor", "", None)?;

    // Mouse is sometimes good
    api::set_option_value("mouse", "a", None)?;

    // Spacing
    api::set_option_value("expandtab", true, None)?;
    api::set_option_value("autoindent", true, None)?;
    api::set_option_value("smartindent", true, None)?;
    api::set_option_value("smarttab", true, None)?;
    api::set_option_value("tabstop", 4, None)?;
    api::set_option_value("shiftwidth", 4, None)?;
    api::set_option_value("joinspaces", false, None)?;

    // Buffers
    api::set_option_value("hidden", true, None)?;

    // Search settings
    api::set_option_value("ignorecase", true, None)?;
    api::set_option_value("smartcase", true, None)?;
    api::set_option_value("showmatch", true, None)?;
    api::set_option_value("incsearch", true, None)?;
    api::set_option_value("hlsearch", true, None)?;
    api::set_option_value("wrapscan", true, None)?;
    api::set_option_value("inccommand", "split", None)?;

    // Read the file if it's changed
    api::set_option_value("autoread", true, None)?;

    // Backspace over lines
    api::set_option_value("backspace", "indent,eol,start", None)?;

    // Menu
    api::set_option_value("wildmenu", true, None)?;
    api::set_option_value("wildmode", "list:longest", None)?;

    // Clipboard
    api::set_option_value("clipboard", "unnamed", None)?;
    api::set_option_value("number", true, None)?;
    api::set_option_value("relativenumber", true, None)?;

    // Directory setup
    //
    // TODO: Figure out stdpath call_function
    // let base_path = api::call_function("stdpath", ("data",))?;
    if let Some(data_dir) = dirs::data_dir() {
        create_and_set_dir("directory", &Path::join(&data_dir, "swaps"))?;
        create_and_set_dir("undodir", &Path::join(&data_dir, "undo"))?;
        create_and_set_dir("backupdir", &Path::join(&data_dir, "backups"))?;
    }
    api::set_option_value("undofile", true, None)?;

    Ok(())
}

fn create_and_set_dir(name: &str, path: &PathBuf) -> oxi::Result<()> {
    if path.exists() || fs::create_dir_all(&path).is_ok() {
        api::set_option_value(name, path.to_str(), None)?;
    } else {
        print!("Failed to create {}", name)
    }
    Ok(())
}
