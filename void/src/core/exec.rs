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

use std::process::Command;

use crate::core::x::Connection;

pub struct Exec<'a> {
    conn: &'a Connection,
}

impl<'a> Exec<'a> {
    pub fn new<'b>(conn: &'a Connection) -> Exec<'a> {
        Exec { conn: conn }
    }
    pub fn focus_close(&self) {}
    pub fn focus_next(&self) {}
    pub fn focus_previous(&self) {}
    pub fn move_next(&self) {}
    pub fn move_previous(&self) {}
    pub fn change_workspace(&self, ws: u16) {}
    pub fn change_layout_next(&self) {}

    pub fn spawn(&self, proc: &str) {
        let cmd = Command::new(proc).spawn().ok();
        cmd.ok_or_else(|| format!("[E] Could not spawn process {}", proc))
            .unwrap();
    }
}
