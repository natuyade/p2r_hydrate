use leptos::prelude::*;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext };
use wasm_bindgen::JsCast;
use feature_extension_for_wasm_bindgen_futures::JsFuture;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::app::SoundSE;

/* staticはグローバル変数でSync(定義したものもSyncである必要がある)
 　グローバル変数は同じプログラム内で値を共有したい場合に使用できる.
   staticは複数スレッドから同時に触れられるがRefCellはSyncではないため複数スレッドからアクセスしてはいけない.
   thread_local!は静的宣言をラップし,スレッドローカル化(スレッド毎に別の値を保存する[◎threadA: static(A),◎threadB: static(B)])
   つまり,複数スレッドからアクセスされても値がスレッド同士で共有されるわけではないため!Syncでも安全に扱える
   thread_local! + RefCellは!atomic(整数,bool等)staticな値をスレッド毎に分けて安全に扱う*/
thread_local! {
    // staticは不変だがRefCellで内部可変性を持たせることで可変に. RefCell::new(None)で初期化
    static SOUND_LOADER: RefCell<Option<SoundLoader>> = const { RefCell::new(None) };
}
// asyncで,soundfileのロードを非同期で処理
pub async fn init_sounds() {
    {
        let mut loader = SoundLoader::new();
        loader.load("pageflip", "/sounds/cardflip.wav").await;
        loader.load("cursoron", "/sounds/cursoron.wav").await;
        // .with()はthread_local!変数にアクセスする(要クロージャ)ために必要(cellは&)
        SOUND_LOADER.with(|cell| {
            // *で実体を取り出しstatic(None)を(Some(loader(書き込み済みSoundLoader)))に書き換える
            *cell.borrow_mut() = Some(loader);
        });
    }
}
// 再生関数
pub fn play(name: &str) {
    // with||アクセス
    SOUND_LOADER.with(|cell| {
        // Some(loader(書き込み済みSoundLoader)) .as_ref()で中身を参照できる<Option<&SoundLoader>> -> name一致で再生 
        if let Some(loader) = cell.borrow().as_ref() {
            loader.play(name);
        }
    });
}

pub struct SoundLoader {
    // ctx = Context entry point
    pub ctx: AudioContext,
    /* HashMapは入れたもの中からkeyが一致するものを探し,対応した値を持ってきます今回はそれをバッファーとして扱う
       今回はサウンドファイル名('static strは"a.wav"等の静的文字列リテラル)をkeyとし,デコード済みのサウンドを格納する*/
    pub buffers: HashMap<&'static str, AudioBuffer>,
}

// implでloadとplayを追加
impl SoundLoader {
    // 初期化処理
    pub fn new() -> Self {
        Self {
            ctx: AudioContext::new().unwrap(),
            buffers: HashMap::new(),
        }
    }
    
    /*  app起動時に一度だけ呼び,
     　 初期化しAudioBufferをデコードしてキャッシュする 
     　 &mut selfはこの関数がこの構造体自体を書き換えるよ.の宣言 */
    pub async fn load(&mut self, name: &'static str, url: &str) {
        /* wasmで動くrustコードはブラウザの中で動いているためOSのfs,networkに直接アクセスできない
           (wasm自体がlocalfileに直接アクセスできないためfetchが必要)
           ネイティブrustの場合rust->OSに直接アクセスできる.
           そのため経由しているブラウザの機能を借りる必要がある.
           そのブラウザの機能を借りれるのがwindow
           rust(wasm)でwindowを扱うときはjsと同じルールで使用する必要がある*/
        // windowを取得
        let window = web_sys::window().unwrap();
        /* fetch_with_strはJavaScriptのPromiseを返す(rustでのfuture)
         　jsの機能を使用した際,返される値はJsValue
           JsValueにはRustのfutureTraitは実装されていないため.awaitが使えない
           JsFuture::from()でPromise(js)をFuture(rust)に変換する->.awaitが使える */
           
        // awaitでfuture(fetch)を起動しurl(path)にHTTPreqをしファイルをサーバーから取得
        let response = JsFuture::from(
            window.fetch_with_str(url)
        ).await.unwrap();
    
        /* array_bufferはResponse型専用のメソッドなので,intoで変換
           array_bufferdでresponseされた物を生のバイト列として取得する処理
           (特定のメソッドを使うために一時的に型変換はこれからも使えるかも) */
        let resp: web_sys::Response = response.into();
        let array_buffer = JsFuture::from(
            resp.array_buffer().unwrap()
        ).await.unwrap();
    
        /* decode_audio_dataはarray_buffer(生のバイト列)をブラウザが再生可能な型(AudioBuffer)にデコードする
        　 ()の中はJsValue(中にある値はjs_sys::ArrayBuffer)をjs_sys::ArrayBufferに変換decode_audio_dataで使える値にしている
           (unchecked_intoは型チェックなしで型変換する.RustからはJsの値はどんな型でもJsValueとしてみえるので
           コード内のjs処理を動かすために.unchecked_into()
           .into()はimpl定義されていないと使えない)*/
        let audio_buffer = JsFuture::from(
            self.ctx.decode_audio_data(
                &array_buffer.unchecked_into()
            ).unwrap()
        ).await.unwrap();
    
        // 受け取ったnameとデコードしたaudio_bufferをHashMapに追加,あとで呼び出せる
        self.buffers.insert(name, audio_buffer.unchecked_into());
    }
    /* HashMapに保存されたサウンド名を呼び出し(&selfで読み取り宣言)
       loadではname:&'static strでplayで&strなのは(今は違う)
       loadではnameをHashMapに保存するが,もしstaticでない場合
       insertしたname: &strが関数終了時に消えてしまう可能性があるためstaticとして受け取る必要がある
       
       playでは関数の中でnameが役目を終わるのでstaticである必要がない*/
    pub fn play(&self, name: &str) {
        /* HashMapから対応したnameのkeyを探し,値のAudioBufferを受け取る見つからなければエラーログ
           HashMapの返り値はOption型なのでSome()で取り出す*/
        let Some(buffer) = self.buffers.get(name) else {
            leptos::logging::log!("Sound not found: {}", name);
            // returnは関数を即時終了させる.つまり指定されたファイルがなければ下の処理が必要ないのでreturn
            return;
        };

        /* create_buffer_sourceは音データを再生するためのノード(AudioBufferSourceNode:一度限りの消費物)を生成する
           deepcloneで音声をクローンするよりはるかに軽量.
           AudioBufferSourceNodeは音データそのものではなく,
           AudioBufferにある音データを参照しSourceNode毎に再生状態を設定し再生できる*/
        // SourceNodeを生成
        let source: AudioBufferSourceNode = self.ctx
            .create_buffer_source()
            .unwrap();

        // 生成したSourceNodeにキャッシュ済みのAudioBufferを割り当て
        source.set_buffer(Some(buffer));

        /* create_gainで音量調節のためのノードを生成
         　.gain()で音量のパラメータにアクセスし,
           .set_value()で設定されたVolumeSignal(SoundSE)の値をgainに適応*/
        let SoundSE { sevlm, .. } = use_context::<SoundSE>().unwrap();
        let gain = self.ctx.create_gain().unwrap();
        gain.gain().set_value(sevlm.get() as f32 / 100.0);

        /* 生成したノードを接続する
           source(buffer)->gain(volume)->destination(speaker)*/
        source.connect_with_audio_node(&gain).unwrap();
        gain.connect_with_audio_node(&self.ctx.destination()).unwrap();

        // sourceノードを接続された他のノードの情報を参照し.start()で再生
        source.start().unwrap();
        // SourceNodeは再生完了後にメモリから解放される
    }
}