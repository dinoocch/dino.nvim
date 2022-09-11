// use nvim_oxi::{
//     object,
//     self as oxi,
//     Dictionary,
//     Object,
//     Function,
//     FromObject,
// };
//
// use nvim_completion_core as completion_core;
// use nvim_completion_lsp as completion_lsp;
//
// use serde::Deserialize;
//
//
// #[derive(Deserialize)]
// struct CompletionApi {
//     // register_source: Function,
//     // accept_first: Function,
//     // accept_selected: Function,
//     // lsp: Dictionary,
//     // register_source: Function
//     // scroll_details: Function
//     // select_next: Function
//     // select_prev: Function
//     // show: Function
//     setup: Function<Dictionary, ()>
// }
//
// impl FromObject for CompletionApi {
//     fn from_obj(obj: Object) -> oxi::Result<Self> {
//         Self::deserialize(object::Deserializer::new(obj))
//     }
// }
//
//
// pub fn setup_completion() -> oxi::Result<()> {
//     completion_core::register_source(completion_lsp::Lsp);
//     // nvim-completion-core is mostly private, so we have to interact with public lua
//     // api instead of rust
//     let public_config = completion_core::build_api();
//
//     Ok(())
// }
