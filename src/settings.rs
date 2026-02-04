use leptos::prelude::*;

use crate::app::SoundSE;
use crate::play_sound;
use crate::soundload::SoundLoader;

pub fn sounds_vlm() -> (ReadSignal<usize>, WriteSignal<usize>) {
    signal(0usize)
}

#[component]
pub fn setting_menu() -> impl IntoView {
    // setting menu検知
    let (settings, set_settings) = signal(false);
    // setting menu animation切り替え用
    let (settings_anim, set_settings_anim) = signal(false);
    // setting tab用
    let (tab_anim, set_tab_anim) = signal(false);
    // mute時にcurrent volumeを保存
    let (vlmcache, set_vlmcache) = signal(0usize);
    // volumeの値を保存しplay時に適応させる
    let SoundSE { sevlm, set_sevlm } = use_context::<SoundSE>().unwrap();
    
    
    
    view! {
        <div class="settings-wrapper">
            <img src="images/setting.webp"
                class="settings-icon"
                class:setting-anim=move || settings_anim.get()
                on:click=move |_| {
                    if !settings.get_untracked() {
                        set_settings.set(true)
                    } else {
                        set_tab_anim.set(true)
                    }
                    set_settings_anim.set(true)
                }
                on:animationend=move |_| set_settings_anim.set(false)
            />
            <Show when=move || settings.get()>
            <div class="stng-container">
                <div class="stng-bg"
                    on:click=move |_| set_tab_anim.set(true)
                >
                </div>
                <div
                    class="settings"
                    class:settings-tab-anim-open=move || settings.get()
                    class:settings-tab-anim-close=move || tab_anim.get()
                    on:animationend=move |_|
                        if tab_anim.get() {
                            set_tab_anim.set(false);
                            set_settings.set(false)
                        }
                >
                <div class="settings-tab">
                    <h1 class="settings-text">"設定"</h1>
                </div>
                <div class="sounds-stng">
                    <img class="close-button"
                        src="images/close.webp"
                        on:click=move |_| set_tab_anim.set(true)
                    />
                    <div
                        class="serange-wrapper"
                    >
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="1"
                        value="0"
                        class="serange"
                        on:click=move |_| set_vlmcache.set(sevlm.get_untracked())
                        on:input:target=move |ev| {
                            set_sevlm.set(ev.target().value().parse::<usize>().unwrap())
                        }
                        // 中の値(dom上でだけ変わる値)≒value=""
                        prop:value=move || sevlm.get()
                    />
                    </div>
                    <button
                        on:click=move |_| {
                            if sevlm.get() > 0{
                                set_sevlm.set(0)
                            } else {
                                set_sevlm.set(vlmcache.get_untracked())
                            }
                        }>
                        <Show when=move || {sevlm.get() > 0}>
                            "🔊"
                        </Show>
                        <Show when=move || sevlm.get() == 0>
                            "🔇"
                        </Show>
                        "SE Volume "{ move || sevlm.get() }"%"
                    </button>
                    <button
                        style="
                            background-color: lime;
                            color: black;
                            padding: 2px;
                        "
                        //on:click=move|_|{}
                    >
                        "クリックでSE再生"
                    </button>
                    </div>
                </div>
            </div>
            </Show>
        </div>
    }
}
