pub mod app;

// macro_rules!
mod p2r_macros;

// resource
mod soundloader;

// css
mod globalcss;

// 共通navigation
mod p2rmenu;
mod settings;
mod sounds;

// hompage
mod homepage;
mod nonesense;
mod pre_date;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(app::App);
}
