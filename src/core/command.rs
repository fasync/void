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

use crate::core::keys;
use std::collections::HashMap;
use std::process::Command;

type Callback = fn();

pub struct Commands {
    keybinds: HashMap<keys::KeyCombo, (Callback, Vec<String>)>,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            keybinds: HashMap::new(),
        }
    }

    pub fn add(self, modkey: &[keys::ModKey], keysym: &str, func: Callback, args: Vec<String>) {
        self.keybinds.insert(
            keys::KeyCombo {
                modmask: modkey[0].mask(),
                keysym,
            },
            (func, args),
        );
    }

    pub fn lookup(self, keycombo: &keys::KeyCombo) {
        let func = self.keybinds.get(keycombo);
        func;
    }

    pub fn exec(self, cmd_str: Vec<String>) {
        let mut cmd = Command::new(&cmd_str[0]);

        for elem in cmd_str.iter().skip(1) {
            cmd.arg(elem);
        }

        cmd.spawn().expect("Command failed to start.");
    }
}
