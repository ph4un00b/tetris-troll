use egui::Window;
use macroquad::prelude::*;

#[macroquad::main("egui with macroquad")]
async fn main() {
    let mut c = 0;
    let mut my_f32 = 0.0;
    let mut my_string = "jamon?";
    let mut my_boolean = false;
    #[derive(PartialEq)]
    enum Enum {
        First,
        Second,
        Third,
    }
    let mut my_enum = Enum::First;
    loop {
        clear_background(SKYBLUE);

        // Process keys, mouse etc.
        //? Conventions❗
        //? https://docs.rs/egui/latest/egui/#conventions
        //? angles are in radians
        //? Vec2::X is right and Vec2::Y is down.
        //? Pos2::ZERO is left top.
        //? Positions and sizes are measured in points. Each point may consist of many physical pixels.
        egui_macroquad::ui(|ctx| {
            Window::new("with sized").show(ctx, |ui| {
                ui.label("Jamon❗ 😏");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut my_enum, Enum::First, "First enum");
                    ui.radio_value(&mut my_enum, Enum::Second, "Second enum");
                    ui.radio_value(&mut my_enum, Enum::Third, "Third enum");
                });

                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        let _ = ui.button("I am becoming wider as needed");
                    },
                );

                ui.add_sized([50.0, 50.0], egui::DragValue::new(&mut my_f32));

                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 100.0;
                    //? remove spacing between widgets
                    //? `radio_value` also works for enums, integers, and more.
                    ui.radio_value(&mut my_boolean, false, "Off");
                    ui.radio_value(&mut my_boolean, true, "On");
                });
            });

            // egui::CentralPanel::default().show(ctx, |ui| {
            //     ui.add(egui::Label::new("Hello World!"));
            //     ui.label("A shorter and more convenient way to add a label. 😊");
            //     if ui.button("Click me").clicked() {
            //         // take some action here
            //     }
            // });
            egui::Window::new("My Allocated Window").show(ctx, |ui| {
                ui.label("Hello World!");
                ui.collapsing("Click to see what is hidden!", |ui| {
                    ui.label("Not much, as it turns out");
                });

                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        let _ = ui.button("I am becoming wider as needed");
                    },
                );

                ui.allocate_space(ui.available_size());
            });
            egui::Area::new("my_area")
                .fixed_pos(egui::pos2(32.0 * 8., 32.0 * 8.))
                .show(ctx, |ui| {
                    ui.label("Floating text!");
                    ui_counter(ui, &mut c);

                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            let _ = ui.button("I am becoming wider as needed");
                        },
                    );
                });

            egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
                ui.label("`Top Bottom Panel`");
                ui.checkbox(&mut my_boolean, "Checkbox");
                ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0).text("My value"));
                if ui
                    .add_enabled(false, egui::Button::new("Can't click this"))
                    .clicked()
                {
                    unreachable!();
                }

                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        let _ = ui.button("I am becoming wider as needed");
                    },
                );

                ui.group(|ui| {
                    ui.label("Within a frame");
                    ui.set_min_height(50.0);
                });

                //? A `scope` creates a temporary [`Ui`] in which you can change settings:
                ui.scope(|ui| {
                    ui.visuals_mut().override_text_color = Some(egui::Color32::KHAKI);
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                    ui.style_mut().wrap = Some(false);

                    ui.label("This text will be kaki, monospace, and won't wrap to a new line");
                }); //? the temporary settings are reverted here
            });
            egui::SidePanel::left("side panel").show(ctx, |ui| {
                ui.label("This is a label");
                ui.hyperlink("https://github.com/emilk/egui");
                ui.text_edit_singleline(&mut my_string);
                if ui.button("Click me").clicked() {}
                ui.separator();
                ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0));
                ui.add(egui::DragValue::new(&mut my_f32));

                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        let _ = ui.button("I am becoming wider as needed");
                    },
                );

                //? A `scope` creates a temporary [`Ui`] in which you can change settings:
                ui.scope(|ui| {
                    ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                    ui.style_mut().wrap = Some(false);

                    ui.label("This text will be red, monospace, and won't wrap to a new line");
                }); //? the temporary settings are reverted here
            });
        });

        // Draw things before egui

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}
