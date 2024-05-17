// Jupyter runtimed handling here

#[allow(unused_imports)]
use anyhow::{Context as _, Result};
#[allow(unused_imports)]
use client::Client;
use editor::Editor;
#[allow(unused_imports)]
use gpui::{actions, AppContext, Context, Global, Model, ModelContext, WeakView};
#[allow(unused_imports)]
use language::language_settings::all_language_settings;
#[allow(unused_imports)]
use settings::SettingsStore;
#[allow(unused_imports)]
use std::sync::Arc;
use ui::prelude::*;
use workspace::Workspace;

actions!(runtimes, [Run]);

/** On startup, we will look for all available kernels, or so I expect */

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            // Note: this will have to both start a kernel if not already running, and run code selections
            workspace.register_action(Runtime::run);
        },
    )
    .detach();
}

pub struct Runtime {
    workspace: WeakView<Workspace>,
}

impl Runtime {
    pub fn run(workspace: &mut Workspace, _: &Run, cx: &mut ViewContext<Workspace>) {
        let code_snippet = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
            .and_then(|editor| {
                let editor = editor.read(cx);
                let range = editor.selections.newest::<usize>(cx).range();
                let buffer = editor.buffer().read(cx).snapshot(cx);

                let start_language = buffer.language_at(range.start);
                let end_language = buffer.language_at(range.end);
                let language_name = if start_language == end_language {
                    start_language.map(|language| language.code_fence_block_name())
                } else {
                    None
                };
                let language_name = language_name.as_deref().unwrap_or("");

                let selected_text = buffer.text_for_range(range).collect::<String>();
                Some(selected_text)
            });

        if let Some(code) = code_snippet {
            println!("Executing code: {}", code);
            // Spawn off at this point
        }
    }
}
