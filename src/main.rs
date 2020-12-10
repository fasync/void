/*
 * Copyright (c) 2020, Florian Buestgens
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     1. Redistributions of source code must retain the above copyright
 *        notice, this list of conditions and the following disclaimer.
 *
 *     2. Redistributions in binary form must reproduce the above copyright notice,
 *        this list of conditions and the following disclaimer in the
 *        documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY Florian Buestgens ''AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL Florian Buestgens BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
mod config;
mod core;
mod layouts;

use crate::core::event::Event;
use crate::core::event::EventLoop;
use crate::core::x::Connection;

#[allow(dead_code)]
#[cfg(target_os = "freebsd")]
mod _restr {
    pub use capsicum::{enter, sandboxed};
}

#[cfg(target_os = "openbsd")]
mod _restr {
    pub use pledge::{pledge, pledge_execpromises, pledge_promises};
}

use _restr::*;

#[cfg(target_os = "freebsd")]
fn _sandbox() {
    enter().expect("[E] Could not sandbox voidwm.");
    assert!(sandboxed(), "[E] Could not sandbox voidwm.");
}

#[cfg(target_os = "openbsd")]
fn _sandbox() {
    pledge_promises![Stdio Exec].unwrap();
}

fn main() {
    // Setup
    _sandbox();

    // Startup
    let mut conn = Connection::open().expect("[E] Could not open connection");
    let mut event_conn = EventLoop::new(&conn);
    let conf = config::Config::new();
    let keys = core::keys::KeyHandlers::new();

    conn.check_wm(&keys).expect("[E] WM is already running.");
    conf.wire();

    let exist_win = conn
        .top_level_windows()
        .expect("[E] Could not determine existing windows.");

    let event_loop = event_conn.get_event_loop();

    for event in event_conn {
        match event {
            Event::MapRequest(window_id) => conn.on_map_request(window_id),
            Event::UnmapNotify(window_id) => conn.on_unmap_notify(&window_id),
            Event::DestroyNotify(window_id) => conn.on_destroy_notify(&window_id),
            Event::KeyPress(key) => conn.on_key_press(key),
            Event::EnterNotify(window_id) => conn.on_enter_notify(&window_id),
        }
    }
}
