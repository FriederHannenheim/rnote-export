use gtk4::gio;
use gio::Cancellable;
use gtk4::glib;
use glib::IsA;
use gtk4::prelude::FileExt;

use std::env;
use std::sync::{Arc, RwLock};
use std::ffi::OsString;

use rnote_engine::render::Renderer;
use rnote_engine::sheet::Sheet;

fn main() {
    let args_os : Vec<OsString> = env::args_os().collect();
    let file = gio::File::for_commandline_arg(&args_os[1]);
    let result = file.load_bytes(Cancellable::NONE);

    if let Ok((file_bytes, _)) = result {
        let mut sheet = Sheet::default();
        sheet.open_sheet_from_rnote_bytes(file_bytes).unwrap();
        print!(
            "{:?}",
            sheet
                .export_sheet_as_xopp_bytes("/tmp/tst", Arc::new(RwLock::new(Renderer::default())))
                .unwrap()
        );
    } else {
        panic!("Could not open file");
    }
}
