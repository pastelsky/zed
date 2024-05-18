use client::Client;
use gpui::App;
use http::HttpClientWithUrl;
use language::language_settings::AllLanguageSettings;
use project::Project;
use settings::SettingsStore;
use std::{path::Path, sync::Arc};

fn main() {
    env_logger::init();

    use clock::FakeSystemClock;

    App::new().run(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        language::init(cx);
        let clock = Arc::new(FakeSystemClock::default());
        let http = Arc::new(HttpClientWithUrl::new("http://localhost:11434", None));

        let client = client::Client::new(clock, http.clone(), cx);
        Client::set_global(client.clone(), cx);

        runtimes::init(cx);
        Project::init_settings(cx);
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |_| {});
        });

        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            eprintln!("Usage: cargo run --example index -p runtimes -- <project_path>");
            cx.quit();
            return;
        }

        cx.spawn(|mut cx| async move {
            let project_path = Path::new(&args[1]);

            let project = Project::example([project_path], &mut cx).await;

            cx.update(|cx| {
                let language_registry = project.read(cx).languages().clone();
                let node_runtime = project.read(cx).node_runtime().unwrap().clone();
                languages::init(language_registry, node_runtime, cx);
            })
            .unwrap();

            cx.background_executor()
                .timer(std::time::Duration::from_secs(100000))
                .await;

            cx.update(|cx| cx.quit()).unwrap();
        })
        .detach();
    });
}
