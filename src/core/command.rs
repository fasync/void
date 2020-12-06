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
    keybinds: HashMap<keys::KeyCombo, Callback>,
}

impl Commands {
    fn new() -> Commands {
        Commands {
            keybinds: HashMap::new(),
        }
    }

    fn add(self, modkey: keys::ModKey, keysym: keys::Key, func: ()) {
        self.keybinds.insert(
            keys::KeyCombo {
                modmask: self.modkey.mask(),
                keysym,
            },
            func,
        );
    }

    fn lookup(self, keycombo: keys::KeyCombo) {
        let func = self.keybinds.get(keycombo);
        func;
    }

    fn exec(self, cmd: str) {
        Command::new(cmd).expect(cmd + " failed to start.");
    }
}
