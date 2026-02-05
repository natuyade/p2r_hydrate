pub mod app;

// macro_rules!
mod p2r_macros;

// resource
mod soundload;

// css
mod globalcss;

// 共通navigation
mod p2rmenu;
mod settings;

// homepage
mod homepage;
mod nonesense;
mod pre_date;

// novel_list
mod novel_list;

// novels
mod novels;
mod page_counter;
mod text_data;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(app::App);
}
