//! LasiusMan desktop entry point.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("lasiusman-io")
        .build()
        .expect("failed to start the async runtime");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(concat!("LasiusMan ", env!("CARGO_PKG_VERSION")))
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native("LasiusMan", options, Box::new(move |cc| Ok(Box::new(lasiusman::app::LasiusManApp::new(cc, runtime)))))
}
