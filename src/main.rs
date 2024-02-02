mod sctk;
pub mod input;
mod counter;
use tokio::time::{self, sleep};
use std::time::Duration;
use sctk::{layer_surface::LayerSurfaceApp};
use smithay_client_toolkit::{reexports::calloop::timer::{TimeoutAction, Timer}, shell::wlr_layer::Anchor};
use tracing_subscriber::EnvFilter;

// Layer Surface App
fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("error"));

    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let (mut state, mut state_loop) = LayerSurfaceApp::new()?;

    state.run((480, 480), Anchor::BOTTOM);

    loop {
        state_loop.dispatch(Duration::ZERO, &mut state)?;
        state.dispatch_loops()?;

        if false {
            break;
        }
    }

    Ok(())
}

// XDG Window
// fn main() -> anyhow::Result<()> {
//     let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));

//     tracing_subscriber::fmt()
//         .compact()
//         .with_env_filter(env_filter)
//         .init();

//     let (mut state, mut state_loop) = XdgWindowApp::new()?;

//     state.run((480, 480));

//     loop {
//         state_loop.dispatch(Duration::ZERO, &mut state)?;
//         state.dispatch_loops()?;

//         if false {
//             break;
//         }
//     }

//     Ok(())
// }
