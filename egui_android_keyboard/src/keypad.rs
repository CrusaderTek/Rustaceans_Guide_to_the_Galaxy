use eframe::egui::{self, Button, Ui, Vec2, pos2, vec2};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Transition {
    #[default]
    None,
    CloseOnNextFrame,
    CloseImmediately,
}

struct KeyPair {
    key: char,
    alt: char,
}
impl KeyPair {
    fn current_key(&self, shift: &bool) -> String {
        if *shift == false {
            return self.key.to_string()
        }
        else {
            return self.alt.to_string()
        }
    }
}
struct KeysLayout {
    row1: Vec<KeyPair>,
    row2: Vec<KeyPair>,
    row3: Vec<KeyPair>,
    row4: Vec<KeyPair>,
}
impl KeysLayout {
    fn default() -> Self{
        Self {
            row1: Vec::from([
                KeyPair{key: '`', alt: '~'},
                KeyPair{key: '1', alt: '!'},
                KeyPair{key: '2', alt: '@'},
                KeyPair{key: '3', alt: '#'},
                KeyPair{key: '4', alt: '$'},
                KeyPair{key: '5', alt: '%'},
                KeyPair{key: '6', alt: '^'},
                KeyPair{key: '7', alt: '&'},
                KeyPair{key: '8', alt: '*'},
                KeyPair{key: '9', alt: '('},
                KeyPair{key: '0', alt: ')'},
                KeyPair{key: '-', alt: '_'},
                KeyPair{key: '=', alt: '+'},
            ]),
            row2: Vec::from([
                KeyPair{key: 'q', alt: 'Q'},
                KeyPair{key: 'w', alt: 'W'},
                KeyPair{key: 'e', alt: 'E'},
                KeyPair{key: 'r', alt: 'R'},
                KeyPair{key: 't', alt: 'T'},
                KeyPair{key: 'y', alt: 'Y'},
                KeyPair{key: 'u', alt: 'U'},
                KeyPair{key: 'i', alt: 'I'},
                KeyPair{key: 'o', alt: 'O'},
                KeyPair{key: 'p', alt: 'P'},
                KeyPair{key: '[', alt: '{'},
                KeyPair{key: ']', alt: '}'},
                KeyPair{key: '\\', alt: '|'},
            ]),
            row3: Vec::from([
                KeyPair{key: 'a', alt: 'A'},
                KeyPair{key: 's', alt: 'S'},
                KeyPair{key: 'd', alt: 'D'},
                KeyPair{key: 'f', alt: 'F'},
                KeyPair{key: 'g', alt: 'G'},
                KeyPair{key: 'h', alt: 'H'},
                KeyPair{key: 'j', alt: 'J'},
                KeyPair{key: 'k', alt: 'K'},
                KeyPair{key: 'l', alt: 'L'},
                KeyPair{key: ';', alt: ':'},
                KeyPair{key: '\'', alt: '"'},
            ]),
            row4: Vec::from([
                KeyPair{key: 'z', alt: 'Z'},
                KeyPair{key: 'x', alt: 'X'},
                KeyPair{key: 'c', alt: 'C'},
                KeyPair{key: 'v', alt: 'V'},
                KeyPair{key: 'b', alt: 'B'},
                KeyPair{key: 'n', alt: 'N'},
                KeyPair{key: 'm', alt: 'M'},
                KeyPair{key: ',', alt: '<'},
                KeyPair{key: '.', alt: '>'},
                KeyPair{key: '/', alt: '?'},
            ]),
        }        
    }    
}

#[derive(Clone, Debug)]
struct State {
    open: bool,
    closable: bool,
    close_on_next_frame: bool,
    start_pos: egui::Pos2,
    focus: Option<egui::Id>,
    events: Option<Vec<egui::Event>>,
}

impl State {
    fn new() -> Self {
        Self {
            open: false,
            closable: false,
            close_on_next_frame: false,
            start_pos: pos2(100.0, 100.0),
            focus: None,
            events: None,
        }
    }

    fn queue_char(&mut self, c: char) {
        let events = self.events.get_or_insert(vec![]);
        if let Some(key) = egui::Key::from_name(&c.to_string()) {
            events.push(egui::Event::Key {
                key,
                physical_key: Some(key),
                pressed: true,
                repeat: false,
                modifiers: Default::default(),
            });
        }
        events.push(egui::Event::Text(c.to_string()));
    }

    fn queue_key(&mut self, key: egui::Key) {
        let events = self.events.get_or_insert(vec![]);
        events.push(egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: Default::default(),
        });
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple keypad widget.
pub struct Keypad {
    id: egui::Id,
    shift: bool,
    keys: KeysLayout,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            id: egui::Id::new("keypad"),
            shift: false,
            keys: KeysLayout::default(),
        }
    }

    pub fn bump_events(&self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        let events = ctx.memory_mut(|m| {
            m.data
                .get_temp_mut_or_default::<State>(self.id)
                .events
                .take()
        });
        if let Some(mut events) = events {
            events.append(&mut raw_input.events);
            raw_input.events = events;
        }
    }

    fn buttons(ui: &mut Ui, state: &mut State, shift: &mut bool, keys: &KeysLayout, screen_rect: egui::Rect) -> Transition {
        let mut trans = Transition::None;
        ui.vertical(|ui| {
            let window_margin = ui.spacing().window_margin;

            let size_1x = vec2((screen_rect.max.x - 32.0) * 0.05, screen_rect.max.y * 0.03);
            let size_15x = vec2((screen_rect.max.x - 32.0) * 0.05 *1.5, screen_rect.max.y * 0.03);
            let size_2x = vec2((screen_rect.max.x - 32.0) * 0.05 *2.0, screen_rect.max.y * 0.03);
            let size_25x = vec2((screen_rect.max.x - 32.0) * 0.05 *2.5, screen_rect.max.y * 0.03);
            let size_3x = vec2((screen_rect.max.x - 32.0) * 0.05 *3.0, screen_rect.max.y * 0.03);
            let size_4x = vec2((screen_rect.max.x - 32.0) * 0.05 *4.0, screen_rect.max.y * 0.03);
            let size_10x = vec2((screen_rect.max.x - 32.0) * 0.05 *10.0, screen_rect.max.y * 0.03);

            let size_1x1 = vec2(32.0, 26.0);
            let _size_1x2 = vec2(32.0, 52.0 + window_margin.topf());
            let _size_2x1 = vec2(64.0 + window_margin.leftf(), 26.0);

            ui.spacing_mut().item_spacing = Vec2::splat(window_margin.leftf());

            
        // });

            ui.horizontal(|ui| {

                if ui.add_sized(size_1x, Button::new("Esc")).clicked() {
                    state.queue_key(egui::Key::Escape);
                }
                if ui.add_sized(size_1x, Button::new("F1")).clicked() {
                    state.queue_key(egui::Key::F1);
                }
                if ui.add_sized(size_1x, Button::new("F2")).clicked() {
                    state.queue_key(egui::Key::F1);
                }
                if ui.add_sized(size_1x, Button::new("F3")).clicked() {
                    state.queue_key(egui::Key::F1);
                }
                if ui.add_sized(size_1x, Button::new("F4")).clicked() {
                    state.queue_key(egui::Key::F1);
                }
                if ui.add_sized(size_1x, Button::new("F5")).clicked() {
                    state.queue_key(egui::Key::F1);
                }
                if ui.add_sized(size_1x, Button::new("F6")).clicked() {
                    state.queue_key(egui::Key::F6);
                }
                if ui.add_sized(size_1x, Button::new("F7")).clicked() {
                    state.queue_key(egui::Key::F7);
                }
                if ui.add_sized(size_1x, Button::new("F8")).clicked() {
                    state.queue_key(egui::Key::F8);
                }
                if ui.add_sized(size_1x, Button::new("F9")).clicked() {
                    state.queue_key(egui::Key::F9);
                }
                if ui.add_sized(size_1x, Button::new("F10")).clicked() {
                    state.queue_key(egui::Key::F10);
                }
                if ui.add_sized(size_1x, Button::new("F11")).clicked() {
                    state.queue_key(egui::Key::F11);
                }
                if ui.add_sized(size_1x, Button::new("F12")).clicked() {
                    state.queue_key(egui::Key::F12);
                }

            });
            ui.horizontal(|ui| {
                for key in &keys.row1 {
                    let cur_key = key.current_key(&shift);
                    let chars: Vec<char> = cur_key.chars().collect();
                    let char = chars[0];
                    if char == '`' || char == '~' {
                        if ui.add_sized(size_15x, Button::new(&*cur_key)).clicked() {
                            state.queue_char(char);
                        }
                    }
                    else{
                        if ui.add_sized(size_1x, Button::new(&*cur_key)).clicked() {
                            state.queue_char(char);
                        }
                    }
                }
                if ui.add_sized(size_3x, Button::new("🔙 Backspace")).clicked() {
                    state.queue_key(egui::Key::Backspace);
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized(size_2x, Button::new("Tab")).clicked() {
                    state.queue_key(egui::Key::Tab);
                }
                for key in &keys.row2 {
                    let cur_key = key.current_key(&shift);
                    let chars: Vec<char> = cur_key.chars().collect();
                    let char = chars[0];
                    if ui.add_sized(size_1x, Button::new(&*cur_key)).clicked() {
                        state.queue_char(char);
                    }
                }
                // if ui.add_sized(size_1x1, Button::new("⏭")).clicked() {
                //     state.queue_key(egui::Key::End);
                // }
            });
            ui.horizontal(|ui| {
                if ui.add_sized(size_25x , Button::selectable(*shift,"Caps")).clicked() {
                    if *shift == true{
                        *shift = false;
                    }
                    else{
                        *shift = true;
                    }
                }
                for key in &keys.row3 {
                    let cur_key = key.current_key(&shift);
                    let chars: Vec<char> = cur_key.chars().collect();
                    let char = chars[0];
                    if ui.add_sized(size_1x, Button::new(&*cur_key)).clicked() {
                        state.queue_char(char);
                    }
                }
                if ui.add_sized(size_3x, Button::new("Enter ⎆")).clicked() {
                    state.queue_key(egui::Key::Enter);
                    // trans = Transition::CloseOnNextFrame;
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized(size_3x, Button::selectable(*shift,"Shift")).clicked() {
                    if *shift == true{
                        *shift = false;
                    }
                    else{
                        *shift = true;
                    }
                }
                for key in &keys.row4 {
                    let cur_key = key.current_key(&shift);
                    let chars: Vec<char> = cur_key.chars().collect();
                    let char = chars[0];
                    if ui.add_sized(size_1x, Button::new(&*cur_key)).clicked() {
                        state.queue_char(char);
                    }
                }
                if ui.add_sized(size_1x, Button::new("⏶")).clicked() {
                    state.queue_key(egui::Key::ArrowUp);
                }
                if ui.add_sized(size_3x, Button::selectable(*shift,"Shift")).clicked() {
                    if *shift == true{
                        *shift = false;
                    }
                    else{
                        *shift = true;
                    }
                }
                // if ui.add_sized(size_1x1, Button::new("⌨")).clicked() {
                //     trans = Transition::CloseImmediately;
                // }
            });
            ui.horizontal(|ui| {
                if ui.add_sized(size_1x, Button::new("Ctrl")).clicked() {
                }
                if ui.add_sized(size_1x, Button::new("")).clicked() {
                }
                if ui.add_sized(size_1x, Button::new("Alt")).clicked() {
                }
                if ui.add_sized(size_10x, Button::new("Space")).clicked() {
                    state.queue_char(' ');
                    // state.queue_key(egui::Key::Space);
                }
                if ui.add_sized(size_1x, Button::new("⏴")).clicked() {
                    state.queue_key(egui::Key::ArrowLeft);
                }
                if ui.add_sized(size_1x, Button::new("⏷")).clicked() {
                    state.queue_key(egui::Key::ArrowDown);
                }
                if ui.add_sized(size_1x, Button::new("⏵")).clicked() {
                    state.queue_key(egui::Key::ArrowRight);
                }
            });
        });

        trans
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let screen_rect = ctx.content_rect();
        let (focus, mut state) = ctx.memory(|m| {
            (
                m.focused(),
                m.data.get_temp::<State>(self.id).unwrap_or_default(),
            )
        });

        let is_first_show = ctx.wants_keyboard_input() && state.focus != focus;
        if is_first_show {
            let y = ctx.style().spacing.interact_size.y * 1.25;
            state.open = true;
            state.start_pos = ctx.input(|i| {
                i.pointer
                    .hover_pos()
                    .map_or(pos2(100.0, 100.0), |p| p + vec2(0.0, y))
            });
            state.focus = focus;
        }

        if state.close_on_next_frame {
            state.open = false;
            state.close_on_next_frame = false;
            state.focus = None;
        }

        let mut open = state.open;

        let win = egui::TopBottomPanel::bottom("");
        // let win = egui::Window::new("⌨ Keypad");
        // let win = if is_first_show {
        //     win.current_pos(state.start_pos)
        // } else {
        //     win.default_pos(state.start_pos)
        // };
        let resp = win
            // .movable(true)
            .resizable(false)
            // .open(&mut open)
            .show(ctx, |ui| {
                ui.set_height(64.0);
                egui::CollapsingHeader::new("⌨  KeyBoard").show(ui, |ui|{
                    Self::buttons(ui, &mut state, &mut self.shift, &KeysLayout::default(), screen_rect);
                    ui.set_height(64.0);
                });
            });

        state.open = open;

        // if let Some(resp) = resp {
        //     match resp.inner {
        //         Some(Transition::CloseOnNextFrame) => {
        //             state.close_on_next_frame = true;
        //         }
        //         Some(Transition::CloseImmediately) => {
        //             state.open = false;
        //             state.focus = None;
        //         }
        //         _ => {}
        //     }
        //     if !state.closable && resp.response.hovered() {
        //         state.closable = true;
        //     }
        //     if state.closable && resp.response.clicked_elsewhere() {
        //         state.open = false;
        //         state.closable = false;
        //         state.focus = None;
        //     }
        //     if is_first_show {
        //         ctx.move_to_top(resp.response.layer_id);
        //     }
        // }

        if let (true, Some(focus)) = (state.open, state.focus) {
            ctx.memory_mut(|m| {
                m.request_focus(focus);
            });
        }

        ctx.memory_mut(|m| m.data.insert_temp(self.id, state));
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new()
    }
}
