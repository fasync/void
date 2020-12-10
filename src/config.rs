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

use crate::core::command::Commands;
use crate::core::keys;

pub struct Config<'a> {
    modkey: keys::ModKey,
    ctrl: &'a Commands,
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        Config {
            // Set your ModKey here!
            modkey: keys::ModKey::Mod4,
            ctrl: &Commands::new(),
        }
    }

    pub fn get_ctrl(self) -> &'a Commands {
        &self.ctrl
    }

    // Set your keycombos here!
    pub fn wire(self) {
        self.ctrl
            .add([self.modkey], "enter", self.ctrl.exec(["stc"]));
        self.ctrl.add([self.modkey], "p", self.ctrl.exec(["dmenu"]));
        self.ctrl.add(
            [self.modkey, keys::ModKey::Shift],
            "d",
            self.ctrl.exec(["stc", "-e", "ranger"]),
        );
    }
}
