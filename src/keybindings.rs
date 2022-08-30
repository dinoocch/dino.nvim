use nvim_oxi::{self as oxi, api};


pub fn setup_keymaps() -> oxi::Result<()> {
    // Space leader
    api::set_var("mapleader", " ")?;

    api::set_keymap(oxi::types::Mode::Normal, "<ESC>", "<cmd>nohlsearch<CR>", None)?;
    api::set_keymap(oxi::types::Mode::Normal, "<leader>q", "<cmd>bdelete<CR>", None)?;
    api::set_keymap(oxi::types::Mode::Normal, "<space><space>", "<cmd>write<CR>", None)?;

    api::set_keymap(oxi::types::Mode::Visual, "<leader><.>", "<cmd>normal .<CR>", None)?;

    // Window related commands (TODO: Make this more explicit probably)
    api::set_keymap(oxi::types::Mode::Normal, "<C-H>", "<C-W><C-H>", None)?;
    api::set_keymap(oxi::types::Mode::Normal, "<C-J>", "<C-W><C-J>", None)?;
    api::set_keymap(oxi::types::Mode::Normal, "<C-K>", "<C-W><C-K>", None)?;
    api::set_keymap(oxi::types::Mode::Normal, "<C-L>", "<C-W><C-L>", None)?;
    Ok(())
}
