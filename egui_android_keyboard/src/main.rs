use droid_ext_bin::MyApp;

// mod perm;

fn main() -> eframe::Result {
    // perm:request_permission();
    eframe::run_native(
        "hello_android",
        Default::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
