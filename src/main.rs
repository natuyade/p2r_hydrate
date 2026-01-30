use axum::{Router, http::HeaderValue, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};

// tokio Async Runtime(axumではasyncを使うので必須)
#[tokio::main]
// サーバーの起動,待ち受け関数. ほかの処理で止まってはいけないのでasync
async fn main() {
    
    // get_comf(None)はどこかにあるmetadeta.leptosを探して適応
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let routes = generate_route_list(App);
    
    // corsの認証設定
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);
    
    // page.
    let app = Router::new()
        .route("/hello-world", get(hello_world))
        // confはConfFileなので中にあるleptos_optionsを取り出し適応させる
        .leptos_routes(&conf.leptos_options, routes, App)
        // StaticFileStream+404err処理
        .fallback(
            // req ssr handler
            leptos_axum::render_app_to_stream_with_context(
                    || {  },
                    // app route
                    || { App },
                )
        )
        .with_state(conf.leptos_options)
        // cors認証をlayerで挟む
        .layer(cors);
    
    // 待ち受けipとポート指定用,127.0.0.1はlocalonly 0.0.0.0で外部からのアクセスを許可
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // TCPポート解放用. awaitはFuture解決まで結果を返さない
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1 style="color: lime">"ハローワールド！"</h1>
    }
}