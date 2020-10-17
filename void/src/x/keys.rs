/**
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

use std::collections::HashMap;
use std::os::raw::c_uint;

type ModMask = c_uint;
type Key = c_uint;

pub enum ModKey {
    Shift,
    Lock,
    Control,
    Mod1,
    Mod2,
    Mod3,
    Mod4,
    Mod5
}

impl ModKey {
    pub fn mask_all() -> ModMask {
        xcb::MOD_MASK_SHIFT
            | xcb::MOD_MASK_LOCK
            | xcb::MOD_MASK_CONTROL
            | xcb::MOD_MASK_1
            | xcb::MOD_MASK_2
            | xcb::MOD_MASK_3
            | xcb::MOD_MASK_4
            | xcb::MOD_MASK_5
    }

    fn mask(self) -> ModMask {
        match self {
            ModKey::Shift => xcb::MOD_MASK_SHIFT,
            ModKey::Lock => xcb::MOD_MASK_LOCK,
            ModKey::Control => xcb::MOD_MASK_CONTROL,
            ModKey::Mod1 => xcb::MOD_MASK_1,
            ModKey::Mod2 => xcb::MOD_MASK_2,
            ModKey::Mod3 => xcb::MOD_MASK_3,
            ModKey::Mod4 => xcb::MOD_MASK_4,
            ModKey::Mod5 => xcb::MOD_MASK_5,
        }
    }
}

pub struct KeyCombo {
    pub mod_mask: ModMask,
    pub keysym: Key,
}

impl KeyCombo {
    fn new(mods: &[ModKey], keysym: Key) -> KeyCombo {
        let mod_mask = mods.iter().fold(0, |mask, mod_key| mask | mod_key.mask());
        KeyCombo { mod_mask, keysym }
    }
}

pub struct KeyHandlers {
    // hashmap: HashMap<KeyCombo, Command>,
}

impl KeyHandlers {
    pub fn key_combos(&self) -> Vec<&KeyCombo> {
        self.hashmap.keys().collect()
    }

    // pub fn get(&self, key_combo: &KeyCombo) -> Option<Command> {
    //     self.hashmap.get(key_combo).cloned()
    // }
}

// impl From<Vec<(Vec<ModKey>, Key, Command)>> for KeyHandlers {
//     fn from(handlers: Vec<(Vec<ModKey>, Key, Command)>) -> KeyHandlers {
//         let mut hashmap = HashMap::new();
//         for (modkeys, keysym, handler) in handlers {
//             hashmap.insert(KeyCombo::new(&modkeys, keysym), handler);
//         }
//         KeyHandlers { hashmap }
//     }
// }
