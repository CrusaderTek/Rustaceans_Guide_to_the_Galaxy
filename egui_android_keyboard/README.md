This is a heavily modified example of the egui keypad example found here: https://github.com/emilk/egui/tree/main/examples/custom_keypad

There is currently not a good way to enter text into egui widgets on android so I caved and made a crappy keyboard.

This is still a work in progress but should run provied you complete the prerequisites: https://github.com/CrusaderTek/Rustaceans_Guide_to_the_Galaxy/blob/main/prerequisits/prerequisites.md
and the work around in: https://github.com/CrusaderTek/Rustaceans_Guide_to_the_Galaxy/tree/main/egui_example
. Keyboard currently does not fit properly on phones but works great on tablets. 


If your havinf problems with enter:
```rust
let line1 = ui.add(egui::TextEdit::singleline(value));
if line1.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)){
    ctx.memory_mut(|mem|{
        mem.move_focus(egui::FocusDirection::Down);
    });
    save_draft = true;
}
```
