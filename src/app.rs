use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;
use crate::p2rmenu::p2r_menu;

use crate::globalcss::global_style;
use crate::homepage::Homepage;
use crate::settings::SettingMenu;
use crate::settings::sounds_vlm;
use crate::soundload;

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
    
    // LocalResourceは初回のみ実行され,futureを処理できその処理状態を値で返せる,signalとしてUI(Suspense等)と結べる
    let sounds_ready = LocalResource::new(|| async {
        soundload::init_sounds().await;
    });
    
    view! {
        /* Suspenseは中のfutureが完了するまで代わりのUIを表示できる
           直接futureを参照してるわけじゃない(Resource,LocalResourceのfutureのみ)*/
        <Suspense attr:style="position:fixed" fallback=|| view! { <p>"Loading Sound Files..."</p> }>
        /* soundfileを事前load(キャッシュに入れる) .map()はOptionがSome()の場合に変数の中にコードが入力されるメソッド
           (やってることはif letと同じ) */
            {move || sounds_ready.get()/*.map(|_| view!{<p>"Loaded!"</p>})}*/}
        </Suspense>
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
