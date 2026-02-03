use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;

use crate::p2rmenu::p2r_menu;

use crate::globalcss::global_style;
use crate::homepage::Homepage;
use crate::settings::SettingMenu;
use crate::settings::sounds_vlm;

#[derive(Clone)]
pub struct SoundSE {
    pub sevlm: ReadSignal<usize>,
    pub set_sevlm: WriteSignal<usize>,
}

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    
    let (sevlm, set_sevlm) = sounds_vlm();
    provide_context(SoundSE { sevlm, set_sevlm });
    
    view! {
        <style>{ global_style() }</style>
        { p2r_menu() }
        { SettingMenu() }
        <Router>
            <Routes fallback = || "Page not found">
                <Route path=path!("/") view=Homepage/>
                <Route path=path!("/homepage") view=Homepage/>
            </Routes>
        </Router>
    }
}
