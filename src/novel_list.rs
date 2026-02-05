use leptos::prelude::*;
use leptos_router::components::A;

use crate::soundload;

// 目次ページ
#[component]
pub fn novel_page_list() -> impl IntoView {
    view! {
            <div class="text-box-pos">
                <div class="text-box">
                    <p>
                        <A
                            attr:class="novel-link"
                            href="/novel_1"
                            on:mouseenter=move |_| soundload::play("cursoron")
                        >"平凡な生活"</A>
                    </p>
                    <p class="p-margin">
                        <A
                            attr:class="novel-link"
                            href="/novel_2"
                            on:mouseenter=move |_| soundload::play("cursoron")
                        >"壊れかけの炒飯"</A>
                    </p>
                    <p class="p-margin">
                        <A
                            attr:class="novel-link"
                            href="/novel_3"
                            on:mouseenter=move |_| soundload::play("cursoron")
                        >"ペンギンgaku園"</A>
                    </p>
                    <p class="list-subtitle">"~リストバンド戦争~"</p>
                </div>
            </div>
    }
}
