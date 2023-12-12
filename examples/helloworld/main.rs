// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .build(tauri::generate_context!(
      "../../examples/helloworld/tauri.conf.json"
    ))
    .expect("error while running tauri application")
    .run(|_app_handle, event| {
      // dbg!(&event);
      match event {
        tauri::RunEvent::ApplicationShouldTerminate { .. } => {
          dbg!("RunEvent::ApplicationShouldTerminate");
        }
        _ => (),
      }
    });
}
