use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn p2r_menu() -> impl IntoView {

    let (menu, set_menu) = signal(false);
    let (menu_anim, set_menu_anim) = signal(false);

    view! {
            <div class="menuwrap">
                <img
                    class="menu-icon"
                    class:menu-anim=move || menu_anim.get()
                    src=move || {
                        if menu.get(){
                            "images/menu_on.webp"
                        } else {
                            "images/menu_off.webp"
                        }
                    }
                    on:click=move |_| {
                        set_menu_anim.set(true);
                        set_menu.set(!menu.get_untracked());
                    }
                    on:animationend=move |_| set_menu_anim.set(false)
                />
                <Show when=move || menu.get()>
                    <div>
                        <nav>
                            <ul>
                                <li class:li-anim1=move || menu.get()>
                                <div 
                                    class="menu-tab-border"
                                    class:li-anim1=move || menu.get()
                                >
                                    <A attr:class="menu-a" href="/"
                                        on:click=move |_| set_menu.set(false)
                                    >"HOME"</A>
                                </div>
                                </li>
                                <li class:li-anim2=move || menu.get()>
                                <div 
                                    class="menu-tab-border"
                                    class:li-anim2=move || menu.get()
                                >
                                    <A attr:class="menu-a" href="/list"
                                        on:click=move |_| set_menu.set(false)
                                    >"目次"</A>
                                </div>
                                </li>
                                <li class:li-anim3=move || menu.get()>
                                <div
                                    class="menu-tab-border"
                                    class:li-anim3=move || menu.get()
                                >
                                    <A attr:class="menu-a" href="/secret"
                                        on:click=move |_| set_menu.set(false)
                                    >"Coming soon"</A>
                                </div>
                                </li>
                            </ul>
                        </nav>
                    </div>
                </Show>
            </div>
    }
}
