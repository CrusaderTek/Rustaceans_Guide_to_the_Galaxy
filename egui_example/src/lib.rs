use eframe::{CreationContext, egui};

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    // Log to android output
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
    .unwrap()
}

// define your egui app see the egui documentation and examples for explaination
pub struct MyApp {
    // VV for running the egui demo VV
    // demo: egui_demo_lib::DemoWindows,
    Label: String,
}

impl MyApp {
    pub fn new(cc: &CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self {
            // VV for running the egui demo VV
            // demo: egui_demo_lib::DemoWindows::default(),
            Label: "testing egui".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // The following was a work around from the folks at egui/eframe I have not tested removing it.
        // Reserve some space at the top so the demo ui isn't hidden behind the android status bar
        // TODO(lucasmerlin): This is a pretty big hack, should be fixed once safe_area implemented
        // for android:
        // https://github.com/rust-windowing/winit/issues/3910
        egui::TopBottomPanel::top("status_bar_space").show(ctx, |ui| {
            ui.set_height(32.0);
        });

        //You can start your egui code here
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui|{
                egui::CollapsingHeader::new("Hello").show(ui, |ui|{
                    ui.label("World!");
                });
            });
        });
        // VV initiallize the demoVV
        // self.demo.ui(ctx);
    }
}
