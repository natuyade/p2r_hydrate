use leptos::{html::Audio, prelude::*};

#[derive(Clone)]
pub struct SoundEffects {
    pub cursoron: NodeRef<Audio>,
    pub pageflip: NodeRef<Audio>,
}

impl SoundEffects {
    pub fn new() -> Self {
        Self {
            cursoron: NodeRef::new(),
            pageflip: NodeRef::new(),
        }
    }
}

#[component]
pub fn load_sounds(
    sound_refs: SoundEffects
) -> impl IntoView {
    view! {
        <audio node_ref=sound_refs.cursoron>
            <source src="sounds/cursoron.mp3" type="audio/mp3"/>
            <source src="sounds/cursoron.ogg" type="audio/ogg"/>
            <source src="sounds/cursoron.wav" type="audio/wav"/>
        </audio>
        <audio node_ref=sound_refs.pageflip>
            <source src="sounds/cardflip.mp3" type="audio/mp3"/>
            <source src="sounds/cardflip.ogg" type="audio/ogg"/>
            <source src="sounds/cardflip.wav" type="audio/wav"/>
        </audio>
    }
}
