pub mod api;
pub mod app;
pub mod pages;

#[cfg(feature = "hydrate")]
use leptos::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(crate::app::App);
}
