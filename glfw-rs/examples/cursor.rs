// Copyright 2013 The GLFW-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate glfw;

use glfw::{Action, Context, CursorMode, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(
            800,
            600,
            "Hello, I am a window.",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_cursor_mode(CursorMode::Disabled);
    window.make_current();

    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::CursorPos(xpos, ypos) => {
            println!("Cursor position: ({:?}, {:?})", xpos, ypos)
        }
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => match window.get_cursor_mode() {
            CursorMode::Disabled => window.set_cursor_mode(CursorMode::Normal),
            CursorMode::Normal => window.set_cursor_mode(CursorMode::Disabled),
            _ => {}
        },
        _ => {}
    }
}
