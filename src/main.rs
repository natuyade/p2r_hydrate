#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::services::ServeDir;
    use tower_http::cors::{Any, CorsLayer};
    use http::HeaderValue;
    
    use p2r_hydrate::app::App;
    
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options.clone();
    let routes = generate_route_list(App);
    
    // cors 認証設定
    let cors = CorsLayer::new()
                .allow_origin("https://p2rush.jp".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any);
    
    let app = axum::Router::new()
        .leptos_routes(&leptos_options, routes, {
                    let _leptos_options = leptos_options.clone();
                    move ||
                    // index.html
                    view! {
                        <!doctype html>
                        <html>
                            <head>
                                <meta charset="utf-8" />
                                <title>じゃれ本部門[P2R]</title>
                                <link rel="icon" href="images/favicon.ico" type="image/x-icon"/>
                                <link rel="icon" href="images/favicon.png" type="image/png"/>
                                <link rel="apple-touch-icon" href="favicon.png"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1" />
                                <meta name="robots" content="noindex, nofollow" />
                                <link
                                    data-trunk
                                    rel="copy-dir"
                                    href="assets"
                                    data-target-path="assets"
                                />
                                <script type="module">
                                    r#"
                                    import init, { hydrate } from '/pkg/p2r_hydrate.js';
                                    init({ module_or_path: '/pkg/p2r_hydrate.wasm' }).then(() => {
                                        console.log('WASM loaded, calling hydrate...');
                                        hydrate();
                                    });
                                    "#
                                </script>
                            </head>
                                <body>
                                    <App/>
                                </body>
                            </html>
                                
                    }
                })
        .fallback(leptos_axum::file_and_error_handler(|_| {
            view! { <App/> }
        }))
        .nest_service("/pkg", ServeDir::new("target/site/pkg"))
        .nest_service("/assets", ServeDir::new("/assets"))
        .with_state(conf.leptos_options)
        // cors層を挟む
        .layer(cors);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
}