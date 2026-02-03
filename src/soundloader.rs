use leptos::prelude::*;
use leptos::logging;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, MediaElementAudioSourceNode };
use feature_extension_for_wasm_bindgen_futures::JsFuture;
#[derive(Clone)]
struct SoundLoader {
    // ctx = Context entry point
    ctx: AudioContext,
    // HashMapは入れたもの中からkeyが一致するものを探し,対応した値を持ってきます
    buffers: std::collections::HashMap<&'static str, AudioBuffer>,
}

impl SoundLoader {
    // 初期化処理
    fn new() -> Self {
        Self {
            ctx: AudioContext::new().unwrap(),
            buffers: std::collections::HashMap::new(),
        }
    }
}

async fn load(&mut self, name: &'static str, url: &str) {
    let resp = logging::log!("Loading: {}", url);
    let response = JsFuture::from(
        window.fetch_with_str(url).unwrap()
    ).await.unwrap();

    let resp: web_sys::Response = response.into();
    let array_buffer = JsFuture::from(
        resp.array_buffer().unwrap()
    ).await.unwrap();

    let audio_buffer = JsFuture::from(
        self.ctx.decode_audio_data_with_array_buffer(
            &array_buffer.unchecked_into()
        ).unwrap()
    ).await.unwrap();

    self.buffers.insert(name, audio_buffer.unchecked_into());
}