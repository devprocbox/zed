use async_dispatcher::{set_dispatcher, Dispatcher, Runnable};
use gpui::{AppContext, PlatformDispatcher};
use project::Fs;
use settings::Settings as _;
use std::{sync::Arc, time::Duration};

mod jupyter_settings;
mod kernels;
mod outputs;
mod repl_editor;
mod repl_sessions_ui;
mod repl_store;
mod session;
mod stdio;

pub use jupyter_settings::JupyterSettings;
pub use kernels::{Kernel, KernelSpecification, KernelStatus};
pub use repl_editor::*;
pub use repl_sessions_ui::{ClearOutputs, Interrupt, ReplSessionsPage, Run, Sessions, Shutdown};
pub use runtimelib::ExecutionState;
pub use session::Session;

use crate::repl_store::ReplStore;

fn zed_dispatcher(cx: &mut AppContext) -> impl Dispatcher {
    struct ZedDispatcher {
        dispatcher: Arc<dyn PlatformDispatcher>,
    }

    // PlatformDispatcher is _super_ close to the same interface we put in
    // async-dispatcher, except for the task label in dispatch. Later we should
    // just make that consistent so we have this dispatcher ready to go for
    // other crates in Zed.
    impl Dispatcher for ZedDispatcher {
        fn dispatch(&self, runnable: Runnable) {
            self.dispatcher.dispatch(runnable, None)
        }

        fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
            self.dispatcher.dispatch_after(duration, runnable);
        }
    }

    ZedDispatcher {
        dispatcher: cx.background_executor().dispatcher.clone(),
    }
}

pub fn init(fs: Arc<dyn Fs>, cx: &mut AppContext) {
    set_dispatcher(zed_dispatcher(cx));
    JupyterSettings::register(cx);
    ::editor::init_settings(cx);
    repl_sessions_ui::init(cx);
    ReplStore::init(fs, cx);
}
