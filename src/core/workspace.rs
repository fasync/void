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

pub struct Workspace<'a, T: Layout<'a>> {
    name: &'a str,
    layout: T,
}

// Impl

impl<'a> Workspace<'a> {
    fn push_master(&self, win: &'a Window) {
        self.master.push(win);
    }

    fn pop_master(&self) -> &'a Window {
        self.master
            .first()
            .ok_or(|| format_err!("[!] No Windows on master"))
            .unwrap()
    }

    fn push_slave(&self, win: &'a Window) {
        self.slave.push(win);
    }

    fn pop_slave(&self) -> &'a Window {
        self.slave
            .first()
            .ok_or(|| format_err!("[!] No Windows on slave"))
            .unwrap()
    }
}
