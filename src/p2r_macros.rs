#[macro_export]
macro_rules! play_sound {
    ($e: expr, $field: ident) => {
        if let Some(audio) = $e.get() {
            let audio_cloned =
                audio
                .clone_node_with_deep(true)
                .unwrap()
                .unchecked_into::<web_sys::HtmlAudioElement>();
            audio_cloned.set_volume($field.get() as f64 / 100.0);
            let _ = audio_cloned.play();
        }
    }
}
