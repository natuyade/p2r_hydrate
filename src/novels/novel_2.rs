use leptos::prelude::*;

use crate::page_counter::{Novel, NovelImg, get_message};
use crate::soundload;

// 小説ページ
#[component]
pub fn novel_page_2() -> impl IntoView {

    /* countはReadSignal, set_countはWriteSignal */
    let (count, set_count) = signal(0usize);

    let page_num = Novel::Novel2.novel_page().len();
    
    view! {
        // count > 0 のときだけ「前」を表示
        <Show when={move || count.get() > 0}>
            <button
                class="button left"
                /* |c|はaddress , *cはその中の実体と思えばいい 
                    実際は||で参照したものを*cで実体化 */
                on:click=
                    move |_| {
                        // view!内のLeptosPrefixはクロージャを一つしか受け取れないため注意
                        soundload::play("pageflip");
                        set_count.update(|c| *c = c.saturating_sub(1))
                    }
            >
                "prev"
            </button>
        </Show>
        // count < pages のときだけ「次」を表示
        <Show when={move || count.get() + 1 < page_num}>
            <button
                class="button right"
                on:click=
                    move |_| {
                        soundload::play("pageflip");
                        set_count.update(|c| *c += 1);
                    }
            >
                "next"
            </button>
        </Show>
        
        <div class="novelbg">
            <div class="inner-bg">
                <div class="inner">
                    <h1>"『平凡な生活』"</h1>
                    <Show 
                        when= move || NovelImg::Novel2.nimgpath(count.get()).is_some()
                        fallback=|| ()
                    >{
                        view!{
                            <img
                                class="illust"
                                src = move || NovelImg::Novel2.nimgpath(count.get())
                            />
                        }
                    }
                    </Show>
                
                    <p class="novel">{ move || get_message(Novel::Novel2 , count.get()) }</p>                
                
                </div>
            </div>
        </div>
    }
}
