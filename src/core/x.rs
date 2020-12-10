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
// crate
use crate::core::keys;
use crate::core::layout::Layout;

// std
use std::collections::HashMap;

// thirdparty
use xcb::randr;
use xcb_util::keysyms::KeySymbols;
use xcb_util::{ewmh, icccm};

// Types
pub type Result<T> = std::result::Result<T, failure::Error>;

// Macros
macro_rules! atoms {
    ( $( $name:ident ),+ ) => {
        struct Atoms {
            $(
                pub $name: xcb::Atom
            ),*
        }

        impl Atoms {
            pub fn new(conn: &xcb::Connection) -> Result<Atoms> {
                Ok(Atoms {
                    $(
                        $name: Connection::get_atom(conn, stringify!($name))?
                    ),*
                })
            }
        }
    };
    ( $( $name:ident ),+ , ) => (atoms!($( $name ),+);)
}

atoms!(WM_DELETE_WINDOW, WM_PROTOCOLS,);

// Enum
pub enum WindowType {
    Desktop,
    Dock,
    Toolbar,
    Menu,
    Utility,
    Splash,
    Dialog,
    DropdownMenu,
    PopupMenu,
    Tooltip,
    Notification,
    Combo,
    Dnd,
    Normal,
}

pub enum WindowState {
    Modal,
    Sticky,
    MaximizedVert,
    MaximizedHorz,
    Shaded,
    SkipTaskbar,
    SkipPager,
    Hidden,
    Fullscreen,
    Above,
    Below,
    DemandsAttention,
}

// Structs
pub struct Window(pub xcb::Window);

pub struct Connection {
    pub conn: ewmh::Connection,
    root: Window,
    atoms: Atoms,
    id: i32,
    window_type_lookup: HashMap<xcb::Atom, WindowType>,
    window_state_lookup: HashMap<xcb::Atom, WindowState>,
}

impl Window {
    pub fn get(&self) -> xcb::Window {
        self.0
    }
}

impl Connection {
    // Public

    // Open connection to the X Server

    pub fn open() -> Result<Connection> {
        let mut types = HashMap::new();
        let mut states = HashMap::new();

        let (conn, id) = xcb::Connection::connect(None)?;
        let conn = ewmh::Connection::connect(conn).map_err(|(e, _)| e)?;
        let root = conn
            .get_setup()
            .roots()
            .nth(id as usize)
            .ok_or_else(|| failure::format_err!("[E] Failed to determine root window"))?
            .root();
        let atoms = Atoms::new(&conn)?;

        Ok(Connection {
            conn,
            root: Window(root),
            atoms,
            id,
            window_type_lookup: types,
            window_state_lookup: states,
        })
    }

    pub fn top_level_windows(&self) -> Result<Vec<Window>> {
        let win_vec = xcb::query_tree(&self.conn, self.root.get())
            .get_reply()?
            .children()
            .iter()
            .map(|w| Window(*w))
            .collect();
        Ok(win_vec)
    }

    // Check if the WM is already running. Register Events.
    pub fn check_wm(&self, handler: &keys::KeyHandlers) -> Result<()> {
        xcb::change_window_attributes_checked(
            &self.conn,
            self.root.get(),
            &[(
                xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY | xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT,
            )],
        )
        .request_check()?;
        Ok(())
    }

    // Window events

    pub fn window_root(&self) -> &Window {
        &self.root
    }

    pub fn window_close(&self, win: &Window) {
        if self
            .query_protocols(win)
            .map(|proto| proto.contains(&self.atoms.WM_DELETE_WINDOW))
            .unwrap_or(false)
        {
            let data = xcb::ClientMessageData::from_data32([
                self.atoms.WM_DELETE_WINDOW,
                xcb::CURRENT_TIME,
                0,
                0,
                0,
            ]);

            let event = xcb::ClientMessageEvent::new(32, win.get(), self.atoms.WM_PROTOCOLS, data);

            xcb::send_event(
                &self.conn,
                false,
                win.get(),
                xcb::EVENT_MASK_NO_EVENT,
                &event,
            );
        } else {
            xcb::destroy_window(&self.conn, win.get());
        }
    }

    pub fn window_configure(&self, win: &Window, x: u32, y: u32, width: u32, height: u32) {
        let val = [
            (xcb::CONFIG_WINDOW_X as u16, x),
            (xcb::CONFIG_WINDOW_Y as u16, y),
            (xcb::CONFIG_WINDOW_WIDTH as u16, width),
            (xcb::CONFIG_WINDOW_HEIGHT as u16, height),
        ];
        xcb::configure_window(&self.conn, win.get(), &val);
    }

    pub fn window_map(&self, win: &Window) {
        xcb::map_window(&self.conn, win.get());
    }

    pub fn window_unmap(&self, win: &Window) {
        xcb::unmap_window(&self.conn, win.get());
    }

    pub fn window_geometry(&self, win: &Window) -> (u32, u32) {
        let reply = xcb::get_geometry(&self.conn, win.get())
            .get_reply()
            .unwrap();

        (u32::from(reply.width()), u32::from(reply.height()))
    }

    pub fn window_enable_keyevents(&self, win: &Window, key_handlers: &keys::KeyHandlers) {
        let ksym = KeySymbols::new(&self.conn);
        for key in key_handlers.key_combos() {
            match ksym.get_keycode(key.keysym).next() {
                Some(keycode) => {
                    xcb::grab_key(
                        &self.conn,
                        false,
                        win.get(),
                        key.mod_mask as u16,
                        keycode,
                        xcb::GRAB_MODE_ASYNC as u8,
                        xcb::GRAB_MODE_ASYNC as u8,
                    );
                }
                None => println!("[E] Could not get keycode: {}", key.keysym),
            }
        }
    }

    pub fn window_enable_tracking(&self, win: &Window) {
        let val = [(
            xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_ENTER_WINDOW | xcb::EVENT_MASK_STRUCTURE_NOTIFY,
        )];
        xcb::change_window_attributes(&self.conn, win.get(), &val);
    }

    pub fn window_disable_tracking(&self, win: &Window) {
        let val = [(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_NO_EVENT)];
        xcb::change_window_attributes(&self.conn, win.get(), &val);
    }

    pub fn window_focus(&self, win: &Window) {
        xcb::set_input_focus(
            &self.conn,
            xcb::INPUT_FOCUS_POINTER_ROOT as u8,
            win.get(),
            xcb::CURRENT_TIME,
        );
        ewmh::set_active_window(&self.conn, self.id, win.get());
    }

    pub fn window_unfocus(&self) {
        ewmh::set_active_window(&self.conn, self.id, xcb::NONE);
    }

    pub fn window_focused(&self) {}

    pub fn get_screen_width(&self) -> u16 {
        randr::get_screen_size_range(&self.conn, self.window_root().get())
            .get_reply()
            .unwrap()
            .max_width()
    }

    pub fn flush(&self) {
        self.conn.flush();
    }

    // Private
    fn query_protocols(&self, win: &Window) -> Result<Vec<xcb::Atom>> {
        let reply =
            icccm::get_wm_protocols(&self.conn, win.get(), self.atoms.WM_PROTOCOLS).get_reply()?;
        Ok(reply.atoms().to_vec())
    }

    fn get_atom(conn: &xcb::Connection, name: &str) -> Result<xcb::Atom> {
        Ok(xcb::intern_atom(conn, false, name).get_reply()?.atom())
    }
}
