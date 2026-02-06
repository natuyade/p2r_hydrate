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
    let (imgwindow, set_imgwindow) = signal(false);
    let (imgwindow_anim, set_imgwindow_anim) = signal(false);
    
    let schedule_date = PRE_DATE[0];
    let schedule_img = PRE_DATE[1];
    
    view! {
        <Show when=move || imgwindow.get()>
            <div class="imgwindow-wrapper">
            <div class="imgwindow-bg"
                class:imgwindow-open=move || imgwindow.get()
                class:imgwindow-close=move || !imgwindow_anim.get()
                on:click=move |_| set_imgwindow_anim.set(false)
            />
                <div class="imgwindow-container"
                    class:imgwindow-open=move || imgwindow.get()
                    class:imgwindow-close=move || !imgwindow_anim.get()
                    on:animationend=move |_| {
                        if !imgwindow_anim.get() {
                            set_imgwindow.set(false)
                        }
                    }
                >
                    <div class="imgwindow">
                    <div class="imgwindow-bg2"
                        on:click=move |_| set_imgwindow_anim.set(false)
                    />
                        <img class="imgwindow-img" src=schedule_img />
                        <img class="imgwindow-closebtn"
                            src="images/close.webp"
                            on:click=move |_| set_imgwindow_anim.set(false)
                        />
                    </div>
                </div>
            </div>
        </Show>
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
                    <img 
                        class="schedule-img" 
                        src=schedule_img
                        on:click=move |_| {
                            set_imgwindow_anim.set(true);
                            set_imgwindow.set(true)
                        }
                    />
                </div>
            </div>
            <p class="credit">"© 2025-2026 natuyade."</p>
            <div class="sns">
                //file downloadになる形や外部サイトの場合<A>ではなく<a>
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