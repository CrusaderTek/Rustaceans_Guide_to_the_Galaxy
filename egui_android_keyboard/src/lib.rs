use eframe::{CreationContext, egui};

use std::fs;
use std::process::Command;
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

mod keypad;
use keypad::Keypad;

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

// define your egui app 
pub struct MyApp {
    Label: String,
    input: String,
    injected: Vec<egui::Event>,
    keypad: Keypad,
    output: String,
}

impl MyApp {
    pub fn new(cc: &CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self {
            Label: "testing egui".to_owned(),
            input: String::new(),
            injected: Vec::new(),
            keypad: Keypad::new(),
            output: String::new(),
        }
    }
}
impl eframe::App for MyApp {
    
    //inject input from keyboard/keypad
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput){
        self.keypad.bump_events(ctx, raw_input);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        // ctx.begin_frame(self.raw_input);
        // The following was a work around from the folks at egui/eframe I have not tested removing it.
        // Reserve some space at the top so the demo ui isn't hidden behind the android status bar
        // TODO(lucasmerlin): This is a pretty big hack, should be fixed once safe_area implemented
        // for android:
        // https://github.com/rust-windowing/winit/issues/3910
        egui::TopBottomPanel::top("status_bar_space").show(ctx, |ui| {
            ui.set_height(32.0);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui|{
                ui.label("Termux:~$");
                let line1 = ui.add(egui::TextEdit::singleline(&mut self.input));
                if line1.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)){
                    //do Stuff on enter
                }
                ui.label(self.output.clone());



            });
            self.keypad.show(ctx);
        });
    }
}

