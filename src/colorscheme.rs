/* Heavily inspired by oxocarbon + gruvbox.nvim!! */

use nvim_oxi::{self as oxi, api, opts::*};

// Simplify highlights because we are defining a lot of groups
macro_rules! highlight {
    ($hlname:expr, $fg:expr, $bg:expr) => {
        api::set_hl(
            0,
            $hlname,
            Some(&SetHighlightOpts::builder().fg($fg).bg($bg).build()),
        )?;
    };
    ($hlname:expr, $fg:expr, $bg:expr, $key:ident) => {
        api::set_hl(
            0,
            $hlname,
            Some(
                &SetHighlightOpts::builder()
                    .fg($fg)
                    .bg($bg)
                    .$key(true)
                    .build(),
            ),
        )?;
    };
}
macro_rules! fhighlight {
    ($hlname:expr, $fg:expr) => {
        api::set_hl(
            0,
            $hlname,
            Some(&SetHighlightOpts::builder().fg($fg).build()),
        )?;
    };
    ($hlname:expr, $fg:expr, $key:ident) => {
        api::set_hl(
            0,
            $hlname,
            Some(&SetHighlightOpts::builder().fg($fg).$key(true).build()),
        )?;
    };
}
macro_rules! bhighlight {
    ($hlname:expr, $bg:expr) => {
        api::set_hl(
            0,
            $hlname,
            Some(&SetHighlightOpts::builder().bg($bg).build()),
        )?;
    };
    ($hlname:expr, $bg:expr, $key:ident) => {
        api::set_hl(
            0,
            $hlname,
            Some(&SetHighlightOpts::builder().bg($bg).$key(true).build()),
        )?;
    };
}
macro_rules! link {
    ($from:expr, $to:expr) => {
        api::set_hl(
            0,
            $from,
            Some(&SetHighlightOpts::builder().link($to).build()),
        )?;
    };
}
macro_rules! formathighlight {
    ($hlname:expr, $key:ident) => {
        api::set_hl(
            0,
            $hlname,
            Some(&SetHighlightOpts::builder().$key(true).build()),
        )?;
    };
}

// str is convenient since we just pass these to neovim...
struct Colors<'a> {
    red: &'a str,
    green: &'a str,
    yellow: &'a str,
    blue: &'a str,
    purple: &'a str,
    aqua: &'a str,
    gray: &'a str,
    orange: &'a str,

    neutral_red: &'a str,
    neutral_green: &'a str,
    neutral_yellow: &'a str,
    neutral_blue: &'a str,
    neutral_purple: &'a str,
    neutral_aqua: &'a str,
    // neutral_orange: &'a str,

    bg0: &'a str,
    bg1: &'a str,
    bg2: &'a str,
    bg3: &'a str,
    bg4: &'a str,

    fg0: &'a str,
    fg1: &'a str,
    fg2: &'a str,
    fg3: &'a str,
    fg4: &'a str,
}

impl<'a> Colors<'a> {
    fn set_terminal_colors(&self) -> oxi::Result<()> {
        // api::set_var(&format!("terminal_color_{x}"), self.colors[x])?;
        api::set_var("terminal_color_0", self.bg0)?;
        api::set_var("terminal_color_8", self.gray)?;
        api::set_var("terminal_color_1", self.neutral_red)?;
        api::set_var("terminal_color_9", self.red)?;
        api::set_var("terminal_color_2", self.neutral_green)?;
        api::set_var("terminal_color_10", self.green)?;
        api::set_var("terminal_color_3", self.neutral_yellow)?;
        api::set_var("terminal_color_11", self.yellow)?;
        api::set_var("terminal_color_4", self.neutral_blue)?;
        api::set_var("terminal_color_12", self.blue)?;
        api::set_var("terminal_color_5", self.neutral_purple)?;
        api::set_var("terminal_color_13", self.purple)?;
        api::set_var("terminal_color_6", self.neutral_aqua)?;
        api::set_var("terminal_color_14", self.aqua)?;
        api::set_var("terminal_color_7", self.fg4)?;
        api::set_var("terminal_color_15", self.fg1)?;

        Ok(())
    }

    fn colors(&self) -> [(&'a str, &'a str); 8] {
        [
            ("Red", self.red),
            ("Green", self.green),
            ("Yellow", self.yellow),
            ("Blue", self.blue),
            ("purple", self.aqua),
            ("Gray", self.gray),
            ("Aqua", self.aqua),
            ("Orange", self.orange),
        ]
    }

    fn set_groups(&self) -> oxi::Result<()> {
        self.set_gruvbox_groups()?;
        self.set_base_groups()?;
        self.set_plugin_groups()?;
        Ok(())
    }

    fn set_gruvbox_groups(&self) -> oxi::Result<()> {
        fhighlight!("GruvboxFg0", self.fg0);
        fhighlight!("GruvboxFg1", self.fg1);
        fhighlight!("GruvboxFg2", self.fg2);
        fhighlight!("GruvboxFg3", self.fg3);
        fhighlight!("GruvboxFg4", self.fg4);
        fhighlight!("GruvboxFg4", self.fg4);

        fhighlight!("GruvboxBg0", self.bg0);
        fhighlight!("GruvboxBg1", self.bg1);
        fhighlight!("GruvboxBg2", self.bg2);
        fhighlight!("GruvboxBg3", self.bg3);
        fhighlight!("GruvboxBg4", self.bg4);
        fhighlight!("GruvboxBg4", self.bg4);

        for (name, color) in self.colors() {
            fhighlight!(&format!("Gruvbox{}", name), color);
            fhighlight!(&format!("Gruvbox{}Bold", name), color, bold);
            highlight!(&format!("Gruvbox{}Sign", name), color, self.bg1);
            api::set_hl(
                0,
                &format!("Gruvbox{}Underline", name),
                Some(
                    &SetHighlightOpts::builder()
                        .special(color)
                        .undercurl(true)
                        .build(),
                ),
            )?;
        }

        Ok(())
    }

    fn set_base_groups(&self) -> oxi::Result<()> {
        highlight!("Normal", self.fg1, self.bg0);

        bhighlight!("CursorLine", self.bg1);
        link!("CursorColumn", "CursorLine");

        highlight!("TabLineFill", self.bg4, self.bg1);
        link!("TabLine", "TabLineFill");

        highlight!("TabLineSel", self.green, self.bg1);
        bhighlight!("MatchParen", self.bg3, bold);
        bhighlight!("ColorColumn", self.bg1);
        fhighlight!("Conceal", self.blue);
        highlight!("CursorLineNr", self.yellow, self.bg1);
        link!("NonText", "GruvboxBg2");
        link!("SpecialKey", "GruvboxFg4");

        bhighlight!("Visual", self.bg3);
        link!("VisualNOS", "Visual");
        highlight!("Search", self.yellow, self.bg0);
        highlight!("IncSearch", self.orange, self.bg0);
        highlight!("QuickFixLine", self.bg0, self.yellow, bold);
        fhighlight!("Underlined", self.blue, underline);
        highlight!("StatusLine", self.bg2, self.fg1);
        highlight!("StatusLineNC", self.bg1, self.fg4);
        highlight!("VertSplit", self.bg3, self.bg0);
        highlight!("WildMenu", self.blue, self.bg2, bold);
        link!("Directory", "GruvBoxGreenBold");
        link!("Title", "GruvboxGreenBold");
        highlight!("ErrorMsg", self.bg0, self.red, bold);
        link!("MoreMsg", "GruvboxYellowBold");
        link!("ModeMsg", "GruvboxYellowBold");
        link!("Question", "GruvboxOrangeBold");
        link!("WarningMsg", "GruvboxRedBold");
        fhighlight!("LineNr", self.bg4);
        fhighlight!("SignColumn", self.bg1);
        highlight!("Folded", self.gray, self.bg1, italic);

        link!("MoreMsg", "GruvboxYellowBold");
        link!("ModeMsg", "GruvboxYellowBold");
        link!("Question", "GruvboxOrangeBold");
        link!("WarningMsg", "GruvboxRedBold");
        fhighlight!("LineNr", self.bg4);
        bhighlight!("SignColumn", self.bg1);
        highlight!("Folded", self.gray, self.bg1, italic);
        highlight!("FoldColumn", self.gray, self.bg1);
        link!("vCursor", "Cursor");
        link!("iCursor", "Cursor");
        link!("lCursor", "Cursor");
        link!("Special", "GruvboxOrange");
        fhighlight!("Comment", self.gray, italic);
        fhighlight!("Todo", self.fg0, bold);
        fhighlight!("Error", self.red, bold);
        link!("Statement", "GruvboxRed");
        link!("Conditional", "GruvboxRed");
        link!("Repeat", "GruvboxRed");
        link!("Label", "GruvboxRed");
        link!("Exception", "GruvboxRed");
        fhighlight!("Operator", self.orange, italic);
        link!("Keyword", "GruvboxRed");
        link!("Identifier", "GruvboxBlue");
        link!("Function", "GruvboxGreenBold");
        link!("PreProc", "GruvboxAqua");
        link!("Include", "GruvboxAqua");
        link!("Define", "GruvboxAqua");
        link!("Macro", "GruvboxAqua");
        link!("PreCondit", "GruvboxAqua");
        link!("Constant", "GruvboxPurple");
        link!("Character", "GruvboxPurple");
        fhighlight!("String", self.green, italic);
        link!("Boolean", "GruvboxPurple");
        link!("Number", "GruvboxPurple");
        link!("Float", "GruvboxPurple");
        link!("Type", "GruvboxYellow");
        link!("StorageClass", "GruvboxOrange");
        link!("Structure", "GruvboxAqua");
        link!("Typedef", "GruvboxYellow");
        highlight!("Pmenu", self.fg1, self.bg2);
        highlight!("PmenuSel", self.bg2, self.blue, bold);
        bhighlight!("PmenuSbar", self.bg2);
        bhighlight!("PmenuThumb", self.bg4);
        highlight!("DiffDelete", self.red, self.bg0);
        highlight!("DiffAdd", self.green, self.bg0);
        highlight!("DiffChange", self.aqua, self.bg0);
        highlight!("DiffText", self.yellow, self.bg0);
        link!("SpellCap", "GruvboxBlueUnderline");
        link!("SpellBad", "GruvboxRedUnderline");
        link!("SpellLocal", "GruvboxAquaUnderline");
        link!("SpellRare", "GruvboxPurpleUnderline");

        Ok(())
    }

    fn set_plugin_groups(&self) -> oxi::Result<()> {
        // TODO: Currently just from gruvbox.nvim with vim replace. Should clean this up...
        // For example grouping the links into logical groups -- say errors for example?
        // LSP Diagnostic
        link!("DiagnosticError", "GruvboxRed");
        link!("DiagnosticSignError", "GruvboxRedSign");
        link!("DiagnosticUnderlineError", "GruvboxRedUnderline");
        link!("DiagnosticWarn", "GruvboxYellow");
        link!("DiagnosticSignWarn", "GruvboxYellowSign");
        link!("DiagnosticUnderlineWarn", "GruvboxYellowUnderline");
        link!("DiagnosticInfo", "GruvboxBlue");
        link!("DiagnosticSignInfo", "GruvboxBlueSign");
        link!("DiagnosticUnderlineInfo", "GruvboxBlueUnderline");
        link!("DiagnosticHint", "GruvboxAqua");
        link!("DiagnosticSignHint", "GruvboxAquaSign");
        link!("DiagnosticUnderlineHint", "GruvboxAquaUnderline");
        link!("DiagnosticFloatingError", "GruvboxRed");
        link!("DiagnosticFloatingWarn", "GruvboxOrange");
        link!("DiagnosticFloatingInfo", "GruvboxBlue");
        link!("DiagnosticFloatingHint", "GruvboxAqua");
        link!("DiagnosticVirtualTextError", "GruvboxRed");
        link!("DiagnosticVirtualTextWarn", "GruvboxYellow");
        link!("DiagnosticVirtualTextInfo", "GruvboxBlue");
        link!("DiagnosticVirtualTextHint", "GruvboxAqua");
        link!("LspReferenceRead", "GruvboxYellowBold");
        link!("LspReferenceText", "GruvboxYellowBold");
        link!("LspReferenceWrite", "GruvboxOrangeBold");
        link!("LspCodeLens", "GruvboxGray");
        // nvim-treesitter
        formathighlight!("TSStrong", bold);
        formathighlight!("TSStrike", strikethrough);
        formathighlight!("TSEmphasis", italic);
        formathighlight!("TSUnderline", underline);
        link!("TSKeywordOperator", "GruvboxRed");
        // gitcommit
        link!("gitcommitSelectedFile", "GruvboxGreen");
        link!("gitcommitDiscardedFile", "GruvboxRed");
        // gitsigns.nvim
        link!("GitSignsAdd", "GruvboxGreenSign");
        link!("GitSignsChange", "GruvboxAquaSign");
        link!("GitSignsDelete", "GruvboxRedSign");
        // termdebug
        // debugPC = { bg = palette.faded_blue },
        bhighlight!("debugPC", self.blue);
        link!("debugBreakpoint", "GruvboxRedSign");
        // vim-startify
        link!("StartifyBracket", "GruvboxFg3");
        link!("StartifyFile", "GruvboxFg1");
        link!("StartifyNumber", "GruvboxBlue");
        link!("StartifyPath", "GruvboxGray");
        link!("StartifySlash", "GruvboxGray");
        link!("StartifySection", "GruvboxYellow");
        link!("StartifySpecial", "GruvboxBg2");
        link!("StartifyHeader", "GruvboxOrange");
        link!("StartifyFooter", "GruvboxBg2");
        link!("StartifyVar", "StartifyPath");
        link!("StartifySelect", "Title");
        // vim-dirvish
        link!("DirvishPathTail", "GruvboxAqua");
        link!("DirvishArg", "GruvboxYellow");
        // netrw
        link!("netrwDir", "GruvboxAqua");
        link!("netrwClassify", "GruvboxAqua");
        link!("netrwLink", "GruvboxGray");
        link!("netrwSymLink", "GruvboxFg1");
        link!("netrwExe", "GruvboxYellow");
        link!("netrwComment", "GruvboxGray");
        link!("netrwList", "GruvboxBlue");
        link!("netrwHelpCmd", "GruvboxAqua");
        link!("netrwCmdSep", "GruvboxFg3");
        link!("netrwVersion", "GruvboxGreen");
        // nerdtree
        link!("NERDTreeDir", "GruvboxAqua");
        link!("NERDTreeDirSlash", "GruvboxAqua");
        link!("NERDTreeOpenable", "GruvboxOrange");
        link!("NERDTreeClosable", "GruvboxOrange");
        link!("NERDTreeFile", "GruvboxFg1");
        link!("NERDTreeExecFile", "GruvboxYellow");
        link!("NERDTreeUp", "GruvboxGray");
        link!("NERDTreeCWD", "GruvboxGreen");
        link!("NERDTreeHelp", "GruvboxFg1");
        link!("NERDTreeToggleOn", "GruvboxGreen");
        link!("NERDTreeToggleOff", "GruvboxRed");
        // coc.nvim
        link!("CocErrorSign", "GruvboxRedSign");
        link!("CocWarningSign", "GruvboxOrangeSign");
        link!("CocInfoSign", "GruvboxBlueSign");
        link!("CocHintSign", "GruvboxAquaSign");
        link!("CocErrorFloat", "GruvboxRed");
        link!("CocWarningFloat", "GruvboxOrange");
        link!("CocInfoFloat", "GruvboxBlue");
        link!("CocHintFloat", "GruvboxAqua");
        link!("CocDiagnosticsError", "GruvboxRed");
        link!("CocDiagnosticsWarning", "GruvboxOrange");
        link!("CocDiagnosticsInfo", "GruvboxBlue");
        link!("CocDiagnosticsHint", "GruvboxAqua");
        link!("CocSelectedText", "GruvboxRed");
        link!("CocCodeLens", "GruvboxGray");
        link!("CocErrorHighlight", "GruvboxRedUnderline");
        link!("CocWarningHighlight", "GruvboxOrangeUnderline");
        link!("CocInfoHighlight", "GruvboxBlueUnderline");
        link!("CocHintHighlight", "GruvboxAquaUnderline");
        // telescope.nvim
        link!("TelescopeNormal", "GruvboxFg1");
        link!("TelescopeSelection", "GruvboxOrangeBold");
        link!("TelescopeSelectionCaret", "GruvboxRed");
        link!("TelescopeMultiSelection", "GruvboxGray");
        link!("TelescopeBorder", "TelescopeNormal");
        link!("TelescopePromptBorder", "TelescopeNormal");
        link!("TelescopeResultsBorder", "TelescopeNormal");
        link!("TelescopePreviewBorder", "TelescopeNormal");
        link!("TelescopeMatching", "GruvboxBlue");
        link!("TelescopePromptPrefix", "GruvboxRed");
        link!("TelescopePrompt", "TelescopeNormal");
        // nvim-cmp
        link!("CmpItemAbbr", "GruvboxFg0");
        link!("CmpItemAbbrDeprecated", "GruvboxFg1");
        link!("CmpItemAbbrMatch", "GruvboxBlueBold");
        link!("CmpItemAbbrMatchFuzzy", "GruvboxBlueUnderline");
        link!("CmpItemMenu", "GruvboxGray");
        link!("CmpItemKindText", "GruvboxOrange");
        link!("CmpItemKindMethod", "GruvboxBlue");
        link!("CmpItemKindFunction", "GruvboxBlue");
        link!("CmpItemKindConstructor", "GruvboxYellow");
        link!("CmpItemKindField", "GruvboxBlue");
        link!("CmpItemKindClass", "GruvboxYellow");
        link!("CmpItemKindInterface", "GruvboxYellow");
        link!("CmpItemKindModule", "GruvboxBlue");
        link!("CmpItemKindProperty", "GruvboxBlue");
        link!("CmpItemKindValue", "GruvboxOrange");
        link!("CmpItemKindEnum", "GruvboxYellow");
        link!("CmpItemKindKeyword", "GruvboxPurple");
        link!("CmpItemKindSnippet", "GruvboxGreen");
        link!("CmpItemKindFile", "GruvboxBlue");
        link!("CmpItemKindEnumMember", "GruvboxAqua");
        link!("CmpItemKindConstant", "GruvboxOrange");
        link!("CmpItemKindStruct", "GruvboxYellow");
        link!("CmpItemKindTypeParameter", "GruvboxYellow");
        link!("diffAdded", "GruvboxGreen");
        link!("diffRemoved", "GruvboxRed");
        link!("diffChanged", "GruvboxAqua");
        link!("diffFile", "GruvboxOrange");
        link!("diffNewFile", "GruvboxYellow");
        link!("diffLine", "GruvboxBlue");
        // html
        link!("htmlTag", "GruvboxAquaBold");
        link!("htmlEndTag", "GruvboxAquaBold");
        link!("htmlTagName", "GruvboxBlue");
        link!("htmlArg", "GruvboxOrange");
        link!("htmlTagN", "GruvboxFg1");
        link!("htmlSpecialTagName", "GruvboxBlue");
        fhighlight!("htmlLink", self.fg4, underline);
        link!("htmlSpecialChar", "GruvboxRed");
        // htmlBold = { fg = colors.fg0, bg = colors.bg0, bold = config.bold },
        // htmlBoldUnderline = { fg = colors.fg0, bg = colors.bg0, bold = config.bold, underline = config.underline },
        // htmlBoldItalic = { fg = colors.fg0, bg = colors.bg0, bold = config.bold, italic = config.italic },
        // htmlBoldUnderlineItalic = {
        //   fg = colors.fg0,
        //   bg = colors.bg0,
        //   bold = config.bold,
        //   italic = config.italic,
        //   underline = config.underline,
        // },
        // htmlUnderline = { fg = colors.fg0, bg = colors.bg0, underline = config.underline },
        // htmlUnderlineItalic = {
        //   fg = colors.fg0,
        //   bg = colors.bg0,
        //   italic = config.italic,
        //   underline = config.underline,
        // },
        // htmlItalic = { fg = colors.fg0, bg = colors.bg0, bold = config.italic },
        // xml
        link!("xmlTag", "GruvboxAquaBold");
        link!("xmlEndTag", "GruvboxAquaBold");
        link!("xmlTagName", "GruvboxBlue");
        link!("xmlEqual", "GruvboxBlue");
        link!("docbkKeyword", "GruvboxAquaBold");
        link!("xmlDocTypeDecl", "GruvboxGray");
        link!("xmlDocTypeKeyword", "GruvboxPurple");
        link!("xmlCdataStart", "GruvboxGray");
        link!("xmlCdataCdata", "GruvboxPurple");
        link!("dtdFunction", "GruvboxGray");
        link!("dtdTagName", "GruvboxPurple");
        link!("xmlAttrib", "GruvboxOrange");
        link!("xmlProcessingDelim", "GruvboxGray");
        link!("dtdParamEntityPunct", "GruvboxGray");
        link!("dtdParamEntityDPunct", "GruvboxGray");
        link!("xmlAttribPunct", "GruvboxGray");
        link!("xmlEntity", "GruvboxRed");
        link!("xmlEntityPunct", "GruvboxRed");
        // clojure
        link!("clojureKeyword", "GruvboxBlue");
        link!("clojureCond", "GruvboxOrange");
        link!("clojureSpecial", "GruvboxOrange");
        link!("clojureDefine", "GruvboxOrange");
        link!("clojureFunc", "GruvboxYellow");
        link!("clojureRepeat", "GruvboxYellow");
        link!("clojureCharacter", "GruvboxAqua");
        link!("clojureStringEscape", "GruvboxAqua");
        link!("clojureException", "GruvboxRed");
        link!("clojureRegexp", "GruvboxAqua");
        link!("clojureRegexpEscape", "GruvboxAqua");
        fhighlight!("clojureTegexpCharClass", self.fg3, bold);
        link!("clojureRegexpMod", "clojureRegexpCharClass");
        link!("clojureRegexpQuantifier", "clojureRegexpCharClass");
        link!("clojureParen", "GruvboxFg3");
        link!("clojureAnonArg", "GruvboxYellow");
        link!("clojureVariable", "GruvboxBlue");
        link!("clojureMacro", "GruvboxOrange");
        link!("clojureMeta", "GruvboxYellow");
        link!("clojureDeref", "GruvboxYellow");
        link!("clojureQuote", "GruvboxYellow");
        link!("clojureUnquote", "GruvboxYellow");
        // C
        link!("cOperator", "GruvboxPurple");
        link!("cppOperator", "GruvboxPurple");
        link!("cStructure", "GruvboxOrange");
        // python
        link!("pythonBuiltin", "GruvboxOrange");
        link!("pythonBuiltinObj", "GruvboxOrange");
        link!("pythonBuiltinFunc", "GruvboxOrange");
        link!("pythonFunction", "GruvboxAqua");
        link!("pythonDecorator", "GruvboxRed");
        link!("pythonInclude", "GruvboxBlue");
        link!("pythonImport", "GruvboxBlue");
        link!("pythonRun", "GruvboxBlue");
        link!("pythonCoding", "GruvboxBlue");
        link!("pythonOperator", "GruvboxRed");
        link!("pythonException", "GruvboxRed");
        link!("pythonExceptions", "GruvboxPurple");
        link!("pythonBoolean", "GruvboxPurple");
        link!("pythonDot", "GruvboxFg3");
        link!("pythonConditional", "GruvboxRed");
        link!("pythonRepeat", "GruvboxRed");
        link!("pythonDottedName", "GruvboxGreenBold");
        // CSS
        link!("cssBraces", "GruvboxBlue");
        link!("cssFunctionName", "GruvboxYellow");
        link!("cssIdentifier", "GruvboxOrange");
        link!("cssClassName", "GruvboxGreen");
        link!("cssColor", "GruvboxBlue");
        link!("cssSelectorOp", "GruvboxBlue");
        link!("cssSelectorOp2", "GruvboxBlue");
        link!("cssImportant", "GruvboxGreen");
        link!("cssVendor", "GruvboxFg1");
        link!("cssTextProp", "GruvboxAqua");
        link!("cssAnimationProp", "GruvboxAqua");
        link!("cssUIProp", "GruvboxYellow");
        link!("cssTransformProp", "GruvboxAqua");
        link!("cssTransitionProp", "GruvboxAqua");
        link!("cssPrintProp", "GruvboxAqua");
        link!("cssPositioningProp", "GruvboxYellow");
        link!("cssBoxProp", "GruvboxAqua");
        link!("cssFontDescriptorProp", "GruvboxAqua");
        link!("cssFlexibleBoxProp", "GruvboxAqua");
        link!("cssBorderOutlineProp", "GruvboxAqua");
        link!("cssBackgroundProp", "GruvboxAqua");
        link!("cssMarginProp", "GruvboxAqua");
        link!("cssListProp", "GruvboxAqua");
        link!("cssTableProp", "GruvboxAqua");
        link!("cssFontProp", "GruvboxAqua");
        link!("cssPaddingProp", "GruvboxAqua");
        link!("cssDimensionProp", "GruvboxAqua");
        link!("cssRenderProp", "GruvboxAqua");
        link!("cssColorProp", "GruvboxAqua");
        link!("cssGeneratedContentProp", "GruvboxAqua");
        // javascript
        link!("javaScriptBraces", "GruvboxFg1");
        link!("javaScriptFunction", "GruvboxAqua");
        link!("javaScriptIdentifier", "GruvboxRed");
        link!("javaScriptMember", "GruvboxBlue");
        link!("javaScriptNumber", "GruvboxPurple");
        link!("javaScriptNull", "GruvboxPurple");
        link!("javaScriptParens", "GruvboxFg3");
        // typescript
        link!("typescriptReserved", "GruvboxAqua");
        link!("typescriptLabel", "GruvboxAqua");
        link!("typescriptFuncKeyword", "GruvboxAqua");
        link!("typescriptIdentifier", "GruvboxOrange");
        link!("typescriptBraces", "GruvboxFg1");
        link!("typescriptEndColons", "GruvboxFg1");
        link!("typescriptDOMObjects", "GruvboxFg1");
        link!("typescriptAjaxMethods", "GruvboxFg1");
        link!("typescriptLogicSymbols", "GruvboxFg1");
        link!("typescriptDocSeeTag", "Comment");
        link!("typescriptDocParam", "Comment");
        link!("typescriptDocTags", "vimCommentTitle");
        link!("typescriptGlobalObjects", "GruvboxFg1");
        link!("typescriptParens", "GruvboxFg3");
        link!("typescriptOpSymbols", "GruvboxFg3");
        link!("typescriptHtmlElemProperties", "GruvboxFg1");
        link!("typescriptNull", "GruvboxPurple");
        link!("typescriptInterpolationDelimiter", "GruvboxAqua");
        // purescript
        link!("purescriptModuleKeyword", "GruvboxAqua");
        link!("purescriptModuleName", "GruvboxFg1");
        link!("purescriptWhere", "GruvboxAqua");
        link!("purescriptDelimiter", "GruvboxFg4");
        link!("purescriptType", "GruvboxFg1");
        link!("purescriptImportKeyword", "GruvboxAqua");
        link!("purescriptHidingKeyword", "GruvboxAqua");
        link!("purescriptAsKeyword", "GruvboxAqua");
        link!("purescriptStructure", "GruvboxAqua");
        link!("purescriptOperator", "GruvboxBlue");
        link!("purescriptTypeVar", "GruvboxFg1");
        link!("purescriptConstructor", "GruvboxFg1");
        link!("purescriptFunction", "GruvboxFg1");
        link!("purescriptConditional", "GruvboxOrange");
        link!("purescriptBacktick", "GruvboxOrange");
        // coffescript
        link!("coffeeExtendedOp", "GruvboxFg3");
        link!("coffeeSpecialOp", "GruvboxFg3");
        link!("coffeeCurly", "GruvboxOrange");
        link!("coffeeParen", "GruvboxFg3");
        link!("coffeeBracket", "GruvboxOrange");
        // ruby
        link!("rubyStringDelimiter", "GruvboxGreen");
        link!("rubyInterpolationDelimiter", "GruvboxAqua");
        link!("rubyDefinedOperator", "rubyKeyword");
        // objc
        link!("objcTypeModifier", "GruvboxRed");
        link!("objcDirective", "GruvboxBlue");
        // go
        link!("goDirective", "GruvboxAqua");
        link!("goConstants", "GruvboxPurple");
        link!("goDeclaration", "GruvboxRed");
        link!("goDeclType", "GruvboxBlue");
        link!("goBuiltins", "GruvboxOrange");
        // lua
        link!("luaIn", "GruvboxRed");
        link!("luaFunction", "GruvboxAqua");
        link!("luaTable", "GruvboxOrange");
        // moonscript
        link!("moonSpecialOp", "GruvboxFg3");
        link!("moonExtendedOp", "GruvboxFg3");
        link!("moonFunction", "GruvboxFg3");
        link!("moonObject", "GruvboxYellow");
        // java
        link!("javaAnnotation", "GruvboxBlue");
        link!("javaDocTags", "GruvboxAqua");
        link!("javaCommentTitle", "vimCommentTitle");
        link!("javaParen", "GruvboxFg3");
        link!("javaParen1", "GruvboxFg3");
        link!("javaParen2", "GruvboxFg3");
        link!("javaParen3", "GruvboxFg3");
        link!("javaParen4", "GruvboxFg3");
        link!("javaParen5", "GruvboxFg3");
        link!("javaOperator", "GruvboxOrange");
        link!("javaVarArg", "GruvboxGreen");
        // elixir
        link!("elixirDocString", "Comment");
        link!("elixirStringDelimiter", "GruvboxGreen");
        link!("elixirInterpolationDelimiter", "GruvboxAqua");
        link!("elixirModuleDeclaration", "GruvboxYellow");
        // scala
        link!("scalaNameDefinition", "GruvboxFg1");
        link!("scalaCaseFollowing", "GruvboxFg1");
        link!("scalaCapitalWord", "GruvboxFg1");
        link!("scalaTypeExtension", "GruvboxFg1");
        link!("scalaKeyword", "GruvboxRed");
        link!("scalaKeywordModifier", "GruvboxRed");
        link!("scalaSpecial", "GruvboxAqua");
        link!("scalaOperator", "GruvboxFg1");
        link!("scalaTypeDeclaration", "GruvboxYellow");
        link!("scalaTypeTypePostDeclaration", "GruvboxYellow");
        link!("scalaInstanceDeclaration", "GruvboxFg1");
        link!("scalaInterpolation", "GruvboxAqua");
        // markdown
        fhighlight!("markdownItalic", self.fg3, italic);
        fhighlight!("markdownBold", self.fg3, bold);
        // markdownBoldItalic = { fg = colors.fg3, bold = config.bold, italic = config.italic },
        link!("markdownBoldItalic", "markdownBold");
        link!("markdownH1", "GruvboxGreenBold");
        link!("markdownH2", "GruvboxGreenBold");
        link!("markdownH3", "GruvboxYellowBold");
        link!("markdownH4", "GruvboxYellowBold");
        link!("markdownH5", "GruvboxYellow");
        link!("markdownH6", "GruvboxYellow");
        link!("markdownCode", "GruvboxAqua");
        link!("markdownCodeBlock", "GruvboxAqua");
        link!("markdownCodeDelimiter", "GruvboxAqua");
        link!("markdownBlockquote", "GruvboxGray");
        link!("markdownListMarker", "GruvboxGray");
        link!("markdownOrderedListMarker", "GruvboxGray");
        link!("markdownRule", "GruvboxGray");
        link!("markdownHeadingRule", "GruvboxGray");
        link!("markdownUrlDelimiter", "GruvboxFg3");
        link!("markdownLinkDelimiter", "GruvboxFg3");
        link!("markdownLinkTextDelimiter", "GruvboxFg3");
        link!("markdownHeadingDelimiter", "GruvboxOrange");
        link!("markdownUrl", "GruvboxPurple");
        link!("markdownUrlTitleDelimiter", "GruvboxGreen");
        fhighlight!("markdownLinkText", self.gray, underline);
        link!("markdownIdDeclaration", "markdownLinkText");
        // haskell
        link!("haskellType", "GruvboxBlue");
        link!("haskellIdentifier", "GruvboxAqua");
        link!("haskellSeparator", "GruvboxFg4");
        link!("haskellDelimiter", "GruvboxOrange");
        link!("haskellOperators", "GruvboxPurple");
        link!("haskellBacktick", "GruvboxOrange");
        link!("haskellStatement", "GruvboxPurple");
        link!("haskellConditional", "GruvboxPurple");
        link!("haskellLet", "GruvboxRed");
        link!("haskellDefault", "GruvboxRed");
        link!("haskellWhere", "GruvboxRed");
        link!("haskellBottom", "GruvboxRedBold");
        link!("haskellImportKeywords", "GruvboxPurpleBold");
        link!("haskellDeclKeyword", "GruvboxOrange");
        link!("haskellDecl", "GruvboxOrange");
        link!("haskellDeriving", "GruvboxPurple");
        link!("haskellAssocType", "GruvboxAqua");
        link!("haskellNumber", "GruvboxAqua");
        link!("haskellPragma", "GruvboxRedBold");
        link!("haskellTH", "GruvboxAquaBold");
        link!("haskellForeignKeywords", "GruvboxGreen");
        link!("haskellKeyword", "GruvboxRed");
        link!("haskellFloat", "GruvboxAqua");
        link!("haskellInfix", "GruvboxPurple");
        link!("haskellQuote", "GruvboxGreenBold");
        link!("haskellShebang", "GruvboxYellowBold");
        link!("haskellLiquid", "GruvboxPurpleBold");
        link!("haskellQuasiQuoted", "GruvboxBlueBold");
        link!("haskellRecursiveDo", "GruvboxPurple");
        link!("haskellQuotedType", "GruvboxRed");
        link!("haskellPreProc", "GruvboxFg4");
        link!("haskellTypeRoles", "GruvboxRedBold");
        link!("haskellTypeForall", "GruvboxRed");
        link!("haskellPatternKeyword", "GruvboxBlue");
        // json
        link!("jsonKeyword", "GruvboxGreen");
        link!("jsonQuote", "GruvboxGreen");
        link!("jsonBraces", "GruvboxFg1");
        link!("jsonString", "GruvboxFg1");
        // mail
        link!("mailQuoted1", "GruvboxAqua");
        link!("mailQuoted2", "GruvboxPurple");
        link!("mailQuoted3", "GruvboxYellow");
        link!("mailQuoted4", "GruvboxGreen");
        link!("mailQuoted5", "GruvboxRed");
        link!("mailQuoted6", "GruvboxOrange");
        link!("mailSignature", "Comment");
        // c#
        link!("csBraces", "GruvboxFg1");
        link!("csEndColon", "GruvboxFg1");
        link!("csLogicSymbols", "GruvboxFg1");
        link!("csParens", "GruvboxFg3");
        link!("csOpSymbols", "GruvboxFg3");
        link!("csInterpolationDelimiter", "GruvboxFg3");
        link!("csInterpolationAlignDel", "GruvboxAquaBold");
        link!("csInterpolationFormat", "GruvboxAqua");
        link!("csInterpolationFormatDel", "GruvboxAquaBold");
        // rust btw
        link!("rustSigil", "GruvboxOrange");
        link!("rustEscape", "GruvboxAqua");
        link!("rustStringContinuation", "GruvboxAqua");
        link!("rustEnum", "GruvboxAqua");
        link!("rustStructure", "GruvboxAqua");
        link!("rustModPathSep", "GruvboxFg2");
        link!("rustCommentLineDoc", "Comment");
        link!("rustDefault", "GruvboxAqua");
        // ocaml
        link!("ocamlOperator", "GruvboxFg1");
        link!("ocamlKeyChar", "GruvboxOrange");
        link!("ocamlArrow", "GruvboxOrange");
        link!("ocamlInfixOpKeyword", "GruvboxRed");
        link!("ocamlConstructor", "GruvboxOrange");
        // lspsaga.nvim
        link!("LspSagaCodeActionTitle", "Title");
        link!("LspSagaCodeActionBorder", "GruvboxFg1");
        fhighlight!("LspSagaCoodeActionContent", self.green, bold);
        link!("LspSagaLspFinderBorder", "GruvboxFg1");
        link!("LspSagaAutoPreview", "GruvboxOrange");
        fhighlight!("TargetWord", self.blue, bold);
        link!("FinderSeparator", "GruvboxAqua");
        link!("LspSagaDefPreviewBorder", "GruvboxBlue");
        link!("LspSagaHoverBorder", "GruvboxOrange");
        link!("LspSagaRenameBorder", "GruvboxBlue");
        link!("LspSagaDiagnosticSource", "GruvboxOrange");
        link!("LspSagaDiagnosticBorder", "GruvboxPurple");
        link!("LspSagaDiagnosticHeader", "GruvboxGreen");
        link!("LspSagaSignatureHelpBorder", "GruvboxGreen");
        link!("SagaShadow", "GruvboxBg0");

        // dashboard-nvim
        link!("DashboardShortCut", "GruvboxOrange");
        link!("DashboardHeader", "GruvboxAqua");
        link!("DashboardCenter", "GruvboxYellow");
        fhighlight!("DashboardFooter", self.purple, italic);

        Ok(())
    }
}

const GRUVBOX_DARK: Colors<'static> = Colors {
    // TODO: soft/hard?
    bg0: "#282828",
    bg1: "#3c3836",
    bg2: "#584945",
    bg3: "#665c54",
    bg4: "#7c6f64",

    red: "#fb4934",
    green: "#b8bb26",
    yellow: "#fabd2f",
    blue: "#83a598",
    purple: "#d3869b",
    aqua: "#8ec07c",
    orange: "#fe8019",
    gray: "#928374",

    neutral_red: "#cc241d",
    neutral_green: "#98971a",
    neutral_yellow: "#d79921",
    neutral_blue: "#458588",
    neutral_purple: "#b16286",
    neutral_aqua: "#689d6a",
    // neutral_orange: "#d65d0e",

    fg0: "#fbf1c7",
    fg1: "#ebdbb2",
    fg2: "#d5c4a1",
    fg3: "#bdae93",
    fg4: "#a89984",
};

pub fn setup_colors() -> oxi::Result<()> {
    api::set_option("termguicolors", true)?;

    let palette = GRUVBOX_DARK;
    palette.set_terminal_colors()?;
    palette.set_groups()?;

    Ok(())
}
