use leptos::prelude::*;

use crate::nonesense::NONS;
use crate::pre_date::PRE_DATE;

fn rnd_sp(splash_num: &[&str]) -> String {
    let splash = fastrand::usize(0..splash_num.len());
    splash_num[splash].to_string()
}

#[component]
pub fn homepage() -> impl IntoView{
    let (splash, _) = signal(rnd_sp(&NONS));
    
    let schedule_date = PRE_DATE[0];
    let schedule_img = PRE_DATE[1];
    
    view! {
        <div class="schedule-wrapper">
            <div class="title">
                <h1>"P2R創作小説"</h1>
            </div>
            <div class="splash">
                <p>{ move || splash.get() }</p>
            </div>
            <div class="schedule-box">
                <div class="schedule">
                    <p style="color: yellow">スケジュール</p>
                    <p style="color: white">"playoff"<span style="color: lime">"{ "{ schedule_date }" }"</span></p>
                    //file downloadになる形や外部サイトの場合<A>ではなく<a>
                    <a href=schedule_img target="_blank">
                        <img class="schedule-img" src=schedule_img ></img>
                    </a>
                </div>
            </div>
            <p class="credit">"© 2025-2026 natuyade."</p>
            <div class="sns">
                <a 
                    style="background-image: url('images/github.webp');"
                    href="https://github.com/natuyade"
                />
                <a 
                    style="background-image: url('images/twitter.webp');"
                    href="https://twitter.com/748da4a5"
                />
                <a 
                    style="background-image: url('images/duolingo.webp');"
                    href="https://www.duolingo.com/profile/natuyade"
                />
            </div>
        </div>
    }
}