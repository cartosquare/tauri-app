use std::process::{Command};
use tauri::command;

use rust_mapnik;
use std::ffi::CString;
use std::ffi::CStr;

#[command]
pub fn hello_world_test(event: String) -> Option<String> {
    println!("hello world test");
    let stdout = hello_world(event);
    Some(stdout)
}

pub fn hello_world(event: String) -> String {
    let output = if cfg!(target_os = "windows") {
      Command::new("cmd")
        .args([
          "/C",
          format!("echo {}", event.to_string()).as_str(),
        ])
        .output()
        .expect("failed to execute process")
    } else {
      Command::new("sh")
        .arg("-c")
        .arg(format!("echo {}", event.to_string()).as_str(),)
        .output()
        .expect("failed to execute process")
    };
    let stdout = String::from_utf8(output.stdout).unwrap();
    return stdout;  
  }

#[command]
pub fn render_map() {
    println!("render map");

    unsafe {
        let input_plugin = CString::new("/usr/local/lib/mapnik/input").unwrap();
        let mut s: *mut std::os::raw::c_char = std::ptr::null_mut();
        rust_mapnik::mapnik_register_datasources(input_plugin.as_ptr(), &mut s);

        println!("loading input plugins: {:?}", s);

        let fonts = CString::new("/usr/local/lib/mapnik/fonts").unwrap();
        rust_mapnik::mapnik_register_fonts(fonts.as_ptr(), &mut s);
        println!("loading fonts: {:?}", s);


        let map = rust_mapnik::mapnik_map(512, 512);
        println!("create map success");

        let xml = CString::new("/Users/atlas/Mirror/tauri/tauri-app/styles/test.xml").unwrap();

        rust_mapnik::mapnik_map_load(map, xml.as_ptr());

        let err = rust_mapnik::mapnik_map_last_error(map);
        if !err.is_null() {
            let c_str: &CStr = unsafe { CStr::from_ptr(err) };
            println!("load map fail: {}", c_str.to_str().unwrap());
        } else {
            println!("load map success");
        }

        rust_mapnik::mapnik_map_zoom_all(map);

        let output = CString::new("/Users/atlas/Mirror/tauri/tauri-app/styles/output.png").unwrap();
        rust_mapnik::mapnik_map_render_to_file(map, output.as_ptr());
        println!("render file: {:?}", rust_mapnik::mapnik_map_last_error(map));

        rust_mapnik::mapnik_map_free(map);
    }
}
