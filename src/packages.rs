/*
* Since it will be hard to remove package dependencies for now
*/
use git2::Repository;
use nvim_oxi::{self as oxi, api};
use std::path::Path;

// Use packer since it has a very nice interface until someone implements one in rust
pub fn setup_packer() -> oxi::Result<()> {
    if let Some(home_dir) = dirs::home_dir() {
        let packer_dir = Path::join(
            &home_dir,
            ".local/share/nvim/site/pack/packer/opt/packer.nvim",
        );
        if packer_dir.exists() {
            api::command("packadd packer.nvim")?;
            return Ok(());
        }
        let packer_url = "https://github.com/wbthomason/packer.nvim";
        match Repository::clone(&packer_url, packer_dir) {
            Ok(_) => {
                // api::cmd(&oxi::types::CmdInfos::builder().cmd("packadd").args(vec!["packer.nvim"]));
                api::command("packadd packer.nvim")?;
                Ok(())
            }
            Err(e) => Err(oxi::Error::Other(format!("Failed to clone packer: {}", e))),
        }
    } else {
        Err(oxi::Error::Other("No home directory found?".to_string()))
    }
}

// This is super ugly, will just start rir someday
pub fn add_packages() -> oxi::Result<()> {
    let lua = oxi::mlua::lua();
    let package_config = lua.load(
        r#"
        require("packer").startup(function()
            -- Manage packer updates
            use({ "wbthomason/packer.nvim", opt = true })

            -- Lua Statusline
            use({
                "feline-nvim/feline.nvim",
                config = function()
                    require("config.statusline")
                end,
                requires = {
                    { "kyazdani42/nvim-web-devicons" },
                    { "lewis6991/gitsigns.nvim" },
                    { "SmiteshP/nvim-gps" },
                },
            })

            use({
                "SmiteshP/nvim-navic",
                requires = "neovim/nvim-lspconfig",
                config = function()
                    require("nvim-gps").setup()
                end,
            })

            -- Use treesitter for colors
            use({
                "nvim-treesitter/nvim-treesitter",
                config = function()
                    require("nvim-treesitter.configs").setup({
                        highlight = { enable = true },
                        incremental_selection = {
                            enable = true,
                            keymaps = {
                                init_selection = "gnn",
                                node_incremental = "grn",
                                scope_incremental = "grc",
                                node_decremental = "grm",
                            },
                        },
                        indent = { enable = true },
                        rainbow = {
                            enable = true,
                        },
                    })
                end,
            })

            use({
                "p00f/nvim-ts-rainbow",
                requires = "nvim-treesitter/nvim-treesitter",
            })

            use({
                "lukas-reineke/indent-blankline.nvim",
                config = function()
                    vim.opt.list = true
                    vim.opt.listchars:append("eol:↴")
                    require("indent_blankline").setup({
                        show_end_of_line = true,
                        -- space_char_blankline = " ",
                        -- show_current_context = true,
                        -- show_current_context_start = true,
                    })
                end,
            })

            -- Improved terminal support
            -- TODO: Consider akinsho/nvim-toggleterm.lua
            use({
                "kassio/neoterm",
                config = function()
                    vim.g.neoterm_autoscroll = 1
                    vim.g.neoterm_default_mod = "belowright"
                    vim.g.neoterm_size = 16
                end,
            })

            -- File explorer
            use({
                "kyazdani42/nvim-tree.lua",
                requires = { "kyazdani42/nvim-web-devicons" },
                config = function()
                    vim.g.nvim_tree_git_hl = 1
                    require("nvim-tree").setup({
                        open_on_setup = false,
                        filters = {
                            custom = { ".git", "node_modules", ".cache" },
                        },
                    })
                    vim.api.nvim_set_keymap("n", "<leader>nt", "<cmd>NvimTreeToggle<CR>", { noremap = true, silent = true })
                end,
            })

            -- Git extensions
            use({
                "tpope/vim-fugitive",
                config = function()
                    vim.api.nvim_exec(
                        [[augroup fugitive
                    autocmd!
                    autocmd BufReadPost fugitive://* set bufhidden=delete
                        augroup END]],
                        false
                    )
                end,
            })
            use({
                "tpope/vim-rhubarb",
                requires = { "tpop/vim-fugitive" },
            })

            use({
                "lewis6991/gitsigns.nvim",
                requires = {
                    "nvim-lua/plenary.nvim",
                },
                config = function()
                    require("gitsigns").setup()
                end,
            })

            -- chdir to a 'root' directory when editing
            use("airblade/vim-rooter")

            -- Pretty fuzzy finder
            use({
                "nvim-telescope/telescope.nvim",
                requires = { { "nvim-lua/popup.nvim" }, { "nvim-lua/plenary.nvim" } },
                config = function()
                    if vim.fn.executable("fd") then
                        vim.api.nvim_set_keymap("n", "<leader>f", "<cmd>Telescope fd<CR>", { noremap = true, silent = true })
                    else
                        vim.api.nvim_set_keymap(
                            "n",
                            "<leader>f",
                            "<cmd>Telescope find_files<CR>",
                            { noremap = true, silent = true }
                        )
                    end

                    vim.api.nvim_set_keymap("n", "<leader>s", "<cmd>Telescope live_grep<CR>", { noremap = true, silent = true })
                    vim.api.nvim_set_keymap("n", "<leader>b", "<cmd>Telescope buffers<CR>", { noremap = true, silent = true })
                    vim.api.nvim_set_keymap("n", "<leader>a", "<cmd>Telescope lsp_code_actions<CR>", { noremap = true, silent = true })
                    vim.api.nvim_set_keymap("n", "<leader>r", "<cmd>Telescope lsp_references<CR>", { noremap = true, silent = true })
                    vim.api.nvim_set_keymap("n", "<leader>d", "<cmd>Telescope lsp_definitions<CR>", { noremap = true, silent = true })
                end,
            })

            use({
                "romgrk/barbar.nvim",
                requires = { "kyazdani42/nvim-web-devicons" },
                config = function()
                    vim.api.nvim_set_keymap("n", "]b", [[<cmd>:BufferNext<CR>]], { noremap = true, silent = true })
                    vim.api.nvim_set_keymap("n", "[b", [[<cmd>:BufferPrevious<CR>]], { noremap = true, silent = true })
                end,
            })

            -- Comment stuff out
            use({
                "terrortylor/nvim-comment",
                config = function()
                    require("nvim_comment").setup()
                end,
            })

            -- Operators for surrounding text
            use("tpope/vim-surround")

            -- Sneak operator s{char}{char}
            use("justinmk/vim-sneak")

            -- Useful bracket bindings
            use("tpope/vim-unimpaired")

            -- Easily align things (most of this is probably done with a fixer though?)
            use({
                "junegunn/vim-easy-align",
                config = function()
                    -- map('x', 'ga', '<Plug>(EasyAlign)', {noremap = false})
                    -- map('n', 'ga', '<Plug>(EasyAlign)', {noremap = false})
                end,
            })

            -- lsp
            use({
                "neovim/nvim-lspconfig",
                config = function()
                    require("config.lsp")
                end,
            })

            use({
                "simrat39/rust-tools.nvim",
                config = function()
                    require("rust-tools").setup({})
                end,
            })

            use({
                "jose-elias-alvarez/null-ls.nvim",
                requires = { "neovim/nvim-lspconfig", "nvim-lua/plenary.nvim" },
                config = function()
                    local lspconfig = require("lspconfig")
                    local null_ls = require("null-ls")
                    null_ls.setup({
                        sources = {
                            null_ls.builtins.formatting.black.with({
                                extra_args = { "--line-length", "120" }
                            }),
                            null_ls.builtins.formatting.deno_fmt,
                            null_ls.builtins.formatting.eslint_d,
                            null_ls.builtins.formatting.nixfmt,
                            null_ls.builtins.formatting.rustywind,
                            null_ls.builtins.formatting.trim_newlines,
                            null_ls.builtins.formatting.trim_whitespace,
                            null_ls.builtins.diagnostics.codespell,
                            null_ls.builtins.diagnostics.cppcheck,
                            null_ls.builtins.diagnostics.eslint_d,
                            null_ls.builtins.code_actions.eslint_d,
                            null_ls.builtins.code_actions.gitsigns,
                        },
                        on_attach = function(client)
                            if client.resolved_capabilities.document_formatting then
                                -- autocmd BufWritePre <buffer> lua vim.lsp.buf.formatting_sync()
                                vim.cmd([[
                                augroup LspFormatting
                                    autocmd! * <buffer>
                                augroup END
                                ]])
                            end
                        end,
                    })
                end,
            })

            use({
                "folke/trouble.nvim",
                config = function()
                    require("trouble").setup({})
                    vim.api.nvim_set_keymap("n", "<leader>xx", "<cmd>TroubleToggle<cr>", { silent = true, noremap = true })
                    vim.api.nvim_set_keymap(
                        "n",
                        "<leader>xw",
                        "<cmd>Trouble workspace_diagnostics<cr>",
                        { silent = true, noremap = true }
                    )
                    vim.api.nvim_set_keymap(
                        "n",
                        "<leader>xd",
                        "<cmd>Trouble document_diagnostics<cr>",
                        { silent = true, noremap = true }
                    )
                    vim.api.nvim_set_keymap("n", "<leader>xl", "<cmd>Trouble loclist<cr>", { silent = true, noremap = true })
                    vim.api.nvim_set_keymap("n", "<leader>xq", "<cmd>Trouble quickfix<cr>", { silent = true, noremap = true })
                    vim.api.nvim_set_keymap("n", "gR", "<cmd>Trouble lsp_references<cr>", { silent = true, noremap = true })
                end,
            })

            -- completion
            use({
                "ms-jpq/coq_nvim",
                requires = { "ms-jpq/coq.artifacts", "ms-jpq/coq.thirdparty" },

                config = function()
                    vim.o.completeopt = "menuone,noinsert,noselect"
                    vim.o.shortmess = vim.o.shortmess .. "c"
                    vim.g.coq_settings = { auto_start = "shut-up" }
                    require("coq_3p")({
                        { src = "nvimlua", short_name = "nLUA" },
                        { src = "vimtex", short_name = "vTEX" },
                        { src = "bc", short_name = "MATH", precision = 6 },
                    })
                    local function t(str)
                        return vim.api.nvim_replace_termcodes(str, true, true, true)
                    end

                    function _G.smart_tab()
                        return vim.fn.pumvisible() == 1 and t("<C-n>") or t("<Tab>")
                    end
                    vim.api.nvim_set_keymap("i", "<Tab>", "v:lua.smart_tab()", { expr = true, noremap = true })
                end,
            })

            -- More syntaxes
            use("sheerun/vim-polyglot")

            -- Prettier wildmenu
            use({
                "gelguy/wilder.nvim",
                requires = {
                    -- This is a hack which only works because fzy-lua-native commits built artifacts...
                    "romgrk/fzy-lua-native",
                    "kyazdani42/nvim-web-devicons",
                },
                config = function()
                    -- " For lua_fzy_highlighter   : requires fzy-lua-native vim plugin found
                    vim.cmd([[
                        call wilder#setup({'modes': [':', '/', '?']})
                        call wilder#set_option('use_python_remote_plugin', 0)
                        call wilder#set_option('pipeline', [
                            \   wilder#branch(
                            \     wilder#cmdline_pipeline({
                            \       'fuzzy': 1,
                            \       'fuzzy_filter': wilder#lua_fzy_filter(),
                            \     }),
                            \     wilder#vim_search_pipeline(),
                            \   ),
                            \ ])
                        call wilder#set_option('renderer', wilder#renderer_mux({
                            \ ':': wilder#popupmenu_renderer({
                            \   'highlighter': wilder#lua_fzy_highlighter(),
                            \   'left': [
                            \     ' ',
                            \     wilder#popupmenu_devicons(),
                            \   ],
                            \   'right': [
                            \     ' ',
                            \     wilder#popupmenu_scrollbar(),
                            \   ],
                            \ }),
                            \ '/': wilder#wildmenu_renderer({
                            \   'highlighter': wilder#lua_fzy_highlighter(),
                            \ }),
                            \ }))
                    ]])
                end,
            })

            use({
                "ggandor/lightspeed.nvim",
                requires = { "tpope/vim-repeat" },
            })

            use({
                "rodjek/vim-puppet",
            })

            use({
                "ThePrimeagen/harpoon",
                requires = {
                    "nvim-lua/plenary.nvim",
                },
                -- TODO: Add keybinds
                config = function()
                    vim.keymap.set("n", "<leader>a", function() require("harpoon.mark").add_file() end, { silent = true, noremap = true })
                    vim.keymap.set("n", "<C-e>", function() require("harpoon.ui").toggle_quick_menu() end, { silent = true, noremap = true })
                end,
            })
        end)
        "#);

    match package_config.exec() {
        Ok(_) => Ok(()),
        Err(e) => Err(oxi::Error::Other(format!(
            "Error configuring packages: {}",
            e
        ))),
    }
}

pub fn setup_packages() -> oxi::Result<()> {
    setup_packer()?;
    add_packages()?;
    Ok(())
}
