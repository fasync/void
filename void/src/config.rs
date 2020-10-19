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

use crate::x;

macro_rules! keys {
    [ $( ([$( $mod:ident ),*], $key:ident, $cmd:expr) ),+ $(,)*] => (
        vec![
            $( (vec![$( $mod ),*],  x11::keysym::$key, $cmd) ),+
        ]
    )
}

pub struct Config {
    space: u16,
    modkey: x::keys::ModKey,
}

impl Config {
    pub fn new() -> Config {
        Config {
            space: 2,
            modkey: x::keys::ModKey::Mod4,
        }
    }

    pub fn register_keys(&self) -> () {
        let shift = x::keys::ModKey::Shift;
        let modkey = self.modkey;

        // Key Config goes here.
        keys![
            // Main Keys
            ([modkey, shift], XK_c, x::exec::focus_close()),
            ([modkey], XK_j, x::exec::focus_next()),
            ([modkey], XK_k, x::exec::focus_previous()),
            ([modkey, shift], XK_j, x::exec::move_next()),
            ([modkey, shift], XK_k, x::exec::move_previous()),
            ([modkey, shift], XK_t, x::exec::change_layout_next()),
            // Custom Keys
            (
                [modkey, shift],
                XK_Return,
                x::exec::spawn("stc").expect("[E] Could not start stc.")
            ),
            (
                [modkey, shift],
                XK_d,
                x::exec::spawn("stc -c ranger").expect("[E] Could not start ranger."),
            )
        ];
    }
}
