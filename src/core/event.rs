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
// crate
use crate::core::keys::KeyCombo;
use crate::core::keys::KeySymbols;
use crate::core::x::Connection;
use crate::core::x::Result;
use crate::core::x::Window;

// Structs
pub struct EventLoop<'a> {
    conn: &'a Connection,
}

pub enum Event {
    MapRequest(Window),
    UnmapNotify(Window),
    DestroyNotify(Window),
    KeyPress(KeyCombo),
    EnterNotify(Window),
}

// Impl

impl<'a> EventLoop<'a> {
    pub fn new(conn: &'a Connection) -> Result<EventLoop<'a>> {
        Ok(EventLoop { conn })
    }

    fn on_configure_request(&self, event: &xcb::ConfigureRequestEvent) -> Option<Event> {
        let val = vec![
            (xcb::CONFIG_WINDOW_X as u16, event.x() as u32),
            (xcb::CONFIG_WINDOW_Y as u16, event.y() as u32),
            (xcb::CONFIG_WINDOW_WIDTH as u16, u32::from(event.width())),
            (xcb::CONFIG_WINDOW_HEIGHT as u16, u32::from(event.height())),
            (
                xcb::CONFIG_WINDOW_BORDER_WIDTH as u16,
                u32::from(event.border_width()),
            ),
            (xcb::CONFIG_WINDOW_SIBLING as u16, event.sibling() as u32),
            (
                xcb::CONFIG_WINDOW_STACK_MODE as u16,
                u32::from(event.stack_mode()),
            ),
        ];
        let filter_val: Vec<_> = val
            .into_iter()
            .filter(|&(mask, _)| mask & event.value_mask() != 0)
            .collect();

        xcb::configure_window(&self.conn.conn, event.window(), &filter_val);

        None
    }

    fn on_destroy_notify(&self, event: &xcb::DestroyNotifyEvent) -> Option<Event> {
        Some(Event::DestroyNotify(Window(event.window())))
    }

    fn on_key_press(&self, event: &xcb::KeyPressEvent) -> Option<Event> {
        let symbols = KeySymbols::new(&self.conn.conn);
        let keysym = symbols.press_lookup_keysym(event, 0);
        let modmask = u32::from(event.state());
        let key = KeyCombo { modmask, keysym };
        Some(Event::KeyPress(key))
    }

    fn on_map_request(&self, event: &xcb::MapRequestEvent) -> Option<Event> {
        Some(Event::MapRequest(Window(event.window())))
    }

    fn on_unmap_notify(&self, event: &xcb::UnmapNotifyEvent) -> Option<Event> {
        if event.event() != self.conn.window_root().get() {
            Some(Event::UnmapNotify(Window(event.window())))
        } else {
            None
        }
    }

    fn on_enter_notify(&self, event: &xcb::EnterNotifyEvent) -> Option<Event> {
        Some(Event::EnterNotify(Window(event.event())))
    }
}

impl<'a> Iterator for EventLoop<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.conn.flush();

            let event = self
                .conn
                .conn
                .wait_for_event()
                .expect("[E] IO error while waiting for event.");

            unsafe {
                let propagate = match event.response_type() {
                    xcb::CONFIGURE_REQUEST => self.on_configure_request(xcb::cast_event(&event)),
                    xcb::MAP_REQUEST => self.on_map_request(xcb::cast_event(&event)),
                    xcb::UNMAP_NOTIFY => self.on_unmap_notify(xcb::cast_event(&event)),
                    xcb::DESTROY_NOTIFY => self.on_destroy_notify(xcb::cast_event(&event)),
                    xcb::KEY_PRESS => self.on_key_press(xcb::cast_event(&event)),
                    xcb::ENTER_NOTIFY => self.on_enter_notify(xcb::cast_event(&event)),
                    _ => None,
                };

                if let Some(propagate_event) = propagate {
                    return Some(propagate_event);
                }
            }
        }
    }
}
