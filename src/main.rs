use gio::Cancellable;
use gtk4::gio;
use gtk4::prelude::FileExt;

use std::env;
use std::io::{self, Write};
use std::sync::{Arc, RwLock};

use rnote_engine::render::Renderer;
use rnote_engine::compose;
use rnote_engine::compose::geometry::AABBHelpers;
use rnote_engine::sheet::Sheet;

use nalgebra::Vector2;

fn main() {
    let mut args = env::args().skip(1);
    let file = gio::File::for_parse_name(&args.next().unwrap());
    let result = file.load_bytes(Cancellable::NONE);

    if let Ok((file_bytes, _)) = result {
        let mut sheet = Sheet::default();
        sheet.open_sheet_from_rnote_bytes(file_bytes).unwrap();
        match args.next().unwrap().as_str() {
            "xopp" => {
                io::stdout()
                    .write_all(
                        &sheet
                        .export_sheet_as_xopp_bytes(
                            "/tmp/tst",
                            Arc::new(RwLock::new(Renderer::default())),
                            )
                        .unwrap(),
                        )
                    .unwrap();
            },
            "svg" => {
                print!("{}", &sheet.export_sheet_as_svg_string().unwrap());
            },
            "cropped_svg" => {
                let state = &mut sheet.strokes_state;
                state.set_selected_keys(&state.keys_sorted_chrono(), true);
                let bounds = state.gen_selection_bounds().unwrap().expand(Vector2::new(20.0,20.0));
                state.set_selected_keys(&state.keys_sorted_chrono(), false);
                
                let svgs = sheet.gen_svgs().unwrap();

                let mut svg_data = svgs
                    .iter()
                    .map(|svg| svg.svg_data.as_str())
                    .collect::<Vec<&str>>()
                    .join("\n");

                svg_data = compose::wrap_svg_root(svg_data.as_str(), Some(bounds), Some(bounds), true);
                print!("{}", &svg_data);
            },
            arg => eprintln!("{} not found. Available options are svg and xopp", arg),
        };
    } else {
        panic!("Could not open file");
    }
}
