//! # DBus interface proxy for: `org.freedesktop.UPower.KbdBacklight`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Interface '/org/freedesktop/UPower/KbdBacklight' from service 'org.freedesktop.UPower' on system bus`.
use cctk::sctk::reexports::calloop;
use cctk::toplevel_info::ToplevelInfo;
use cosmic::iced;
use cosmic::iced::subscription;
use cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    SinkExt, StreamExt,
};
use std::{fmt::Debug, hash::Hash, thread::JoinHandle};

use crate::wayland_handler::wayland_handler;

pub fn wayland_subscription<I: 'static + Hash + Copy + Send + Sync + Debug>(
    id: I,
) -> iced::Subscription<WaylandUpdate> {
    subscription::channel(id, 50, move |mut output| async move {
        let mut state = State::Ready;

        loop {
            state = start_listening(state, &mut output).await;
        }
    })
}

pub enum State {
    Ready,
    Waiting(
        UnboundedReceiver<WaylandUpdate>,
        calloop::channel::Sender<WaylandRequest>,
        JoinHandle<()>,
    ),
    Finished,
}

async fn start_listening(
    state: State,
    output: &mut futures::channel::mpsc::Sender<WaylandUpdate>,
) -> State {
    match state {
        State::Ready => {
            let (calloop_tx, calloop_rx) = calloop::channel::channel();
            let (toplevel_tx, toplevel_rx) = unbounded();
            let handle = std::thread::spawn(move || {
                wayland_handler(toplevel_tx, calloop_rx);
            });
            let tx = calloop_tx.clone();
            _ = output.send(WaylandUpdate::Init(tx)).await;
            State::Waiting(toplevel_rx, calloop_tx, handle)
        }
        State::Waiting(mut rx, tx, handle) => {
            if handle.is_finished() {
                _ = output.send(WaylandUpdate::Finished).await;
                return State::Finished;
            }
            match rx.next().await {
                Some(u) => {
                    _ = output.send(u).await;
                    State::Waiting(rx, tx, handle)
                }
                None => {
                    _ = output.send(WaylandUpdate::Finished).await;
                    State::Finished
                }
            }
        }
        State::Finished => iced::futures::future::pending().await,
    }
}

#[derive(Clone, Debug)]
pub enum WaylandUpdate {
    Init(calloop::channel::Sender<WaylandRequest>),
    Finished,
    Toplevel(ToplevelUpdate),
    ActivationToken { token: Option<String>, exec: String },
}

#[derive(Clone, Debug)]
pub enum ToplevelUpdate {
    AddToplevel(ZcosmicToplevelHandleV1, ToplevelInfo),
    UpdateToplevel(ZcosmicToplevelHandleV1, ToplevelInfo),
    RemoveToplevel(ZcosmicToplevelHandleV1),
}

#[derive(Clone, Debug)]
pub enum WaylandRequest {
    Toplevel(ToplevelRequest),
    TokenRequest { app_id: String, exec: String },
}

#[derive(Debug, Clone)]
pub enum ToplevelRequest {
    Activate(ZcosmicToplevelHandleV1),
    Quit(ZcosmicToplevelHandleV1),
    Exit,
}