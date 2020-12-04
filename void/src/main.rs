/*
 * Copyright (c) 2020, Florian Büstgens
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
 * THIS SOFTWARE IS PROVIDED BY <copyright holder> ''AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL <copyright holder> BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

mod core;
mod layouts;

mod config;

use crate::core::x;

fn pledge() {}

fn capsicum() {}

fn main() {
    // Setup
    match std::env::consts::OS {
        "openbsd" => pledge(),
        "freebsd" => capsicum(),
        _ => (),
    }

    // Startup
    let mut conn = x::Connection::open().expect("[E] Could not open connection");
    let mut event_conn = x::EventLoop::new(&conn);
    let exec = core::exec::Exec::new(&conn);
    let conf = config::Config::new(&exec);
    let keys = core::keys::KeyHandlers::new();

    conn.check_wm(&keys).expect("[E] WM is already running.");

    let exist_win = conn
        .top_level_windows()
        .expect("[E] Could not determine existing windows.");

    for win in exist_win {}

    //let event_loop = event_conn.get_event_loop();

    for event in event_conn {
        // match event {
        //     // x::Event::MapRequest(window_id) => on_map_request(window_id),
        //     // x::Event::UnmapNotify(window_id) => self.on_unmap_notify(&window_id),
        //     // x::Event::DestroyNotify(window_id) => self.on_destroy_notify(&window_id),
        //     // x::Event::KeyPress(key) => self.on_key_press(key),
        //     // x::Event::EnterNotify(window_id) => self.on_enter_notify(&window_id),
        // }
    }
}
