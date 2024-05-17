// Jupyter runtimed handling here

#[allow(unused_imports)]
use anyhow::{Context as _, Result};
use client::Client;
use gpui::{AppContext, Global, Model, ModelContext};
use language::language_settings::all_language_settings;
use settings::SettingsStore;
use std::sync::Arc;
use ui::prelude::*;

/** On startup, we will look for all aavailable  */

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let runtimes_provider = cx.new_model(|_| Runtimes::Starting);
    Runtimes::set_global(runtimes_provider.clone(), cx);

    let runtimes_enabled = all_language_settings(None, cx).enable_runtimes;

    if runtimes_enabled {
        runtimes_provider.update(cx, |runtimes_provider, cx| {
            runtimes_provider.start(client.clone(), cx)
        });
    }

    cx.observe_global::<SettingsStore>(move |cx| {
        let runtimes_enabled = all_language_settings(None, cx).enable_runtimes;
        if runtimes_enabled {
            runtimes_provider.update(cx, |runtimes_provider, cx| {
                runtimes_provider.start(client.clone(), cx)
            });
        } else {
            runtimes_provider.update(cx, |runtimes_provider, _cx| runtimes_provider.stop());
        }
    })
    .detach();
}

pub enum Runtimes {
    Starting,
}

#[derive(Clone)]
struct RuntimesGlobal(Model<Runtimes>);

impl Global for RuntimesGlobal {}

impl Runtimes {
    pub fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<RuntimesGlobal>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(runtimes: Model<Self>, cx: &mut AppContext) {
        cx.set_global(RuntimesGlobal(runtimes));
    }

    pub fn start(&mut self, client: Arc<Client>, cx: &mut ModelContext<Self>) {}

    pub fn stop(&mut self) {}
}
