use device_query::{DeviceQuery, DeviceState, Keycode};
use eframe::egui;
use rdev::{Button, Key};

use std::sync::atomic::Ordering;

use crate::app::{AutoclickerApp, AutoclickerMode};
use crate::utils::sanitize_string;

impl eframe::App for AutoclickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();

        sanitize_string(&mut self.s_str, 5);
        sanitize_string(&mut self.ms_str, 5);
        sanitize_string(&mut self.hold_s_str, 5);
        sanitize_string(&mut self.hold_ms_str, 5);

        self.s = self.s_str.parse().unwrap_or_default();
        self.ms = self.ms_str.parse().unwrap_or_default();
        self.hold_s = self.hold_s_str.parse().unwrap_or_default();
        self.hold_ms = self.hold_ms_str.parse().unwrap_or_default();

        let running = self.running.load(Ordering::Relaxed);

        if keys.contains(&Keycode::F6) {
            self.hotkey_pressed = true;
        } else if self.hotkey_pressed {
            self.hotkey_pressed = false;

            if running {
                self.stop();
            } else {
                self.start();
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0].with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.hyperlink_to("Autoclicker", "https://github.com/ThomasAlban/autoclicker");
                    ui.label(" by ");
                    ui.hyperlink_to("ThomasAlban", "https://github.com/ThomasAlban");
                });
                columns[1].with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if running {
                        ui.label("Running");
                    } else {
                        ui.label("Stopped");
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // mode
            ui.horizontal_wrapped(|ui| {
                ui.label("Mode");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if running {
                        ui.set_enabled(false);
                    };
                    ui.selectable_value(&mut self.mode, AutoclickerMode::KeyPress, "Key Press");
                    if running {
                        ui.set_enabled(false);
                    };
                    ui.selectable_value(&mut self.mode, AutoclickerMode::Click, "Click");
                });
            });

            ui.separator();

            // mouse/key type
            if self.mode != AutoclickerMode::KeyPress {
                // mouse button
                ui.horizontal_wrapped(|ui| {
                    ui.label("Mouse Button");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if running {
                            ui.set_enabled(false);
                        };
                        ui.selectable_value(&mut self.click_type, Button::Right, "Right");
                        if running {
                            ui.set_enabled(false);
                        };
                        ui.selectable_value(&mut self.click_type, Button::Left, "Left");
                    });
                });
            } else {
                // key
                ui.horizontal_wrapped(|ui| {
                    ui.label("Key");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if running {
                            ui.set_enabled(false);
                        };
                        egui::ComboBox::from_id_source("Key Select")
                            .selected_text(format!("{:?}", self.key_press_type))
                            .width(105.)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.key_press_type, Key::KeyA, "A");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyB, "B");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyC, "C");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyD, "D");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyE, "E");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyF, "F");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyG, "G");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyH, "H");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyI, "I");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyJ, "J");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyK, "K");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyL, "L");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyM, "M");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyN, "N");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyO, "O");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyP, "P");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyQ, "Q");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyR, "R");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyS, "S");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyT, "T");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyU, "U");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyV, "V");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyW, "W");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyX, "X");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyY, "Y");
                                ui.selectable_value(&mut self.key_press_type, Key::KeyZ, "Z");

                                ui.selectable_value(&mut self.key_press_type, Key::Num0, "0");
                                ui.selectable_value(&mut self.key_press_type, Key::Num1, "1");
                                ui.selectable_value(&mut self.key_press_type, Key::Num2, "2");
                                ui.selectable_value(&mut self.key_press_type, Key::Num3, "3");
                                ui.selectable_value(&mut self.key_press_type, Key::Num4, "4");
                                ui.selectable_value(&mut self.key_press_type, Key::Num5, "5");
                                ui.selectable_value(&mut self.key_press_type, Key::Num6, "6");
                                ui.selectable_value(&mut self.key_press_type, Key::Num7, "7");
                                ui.selectable_value(&mut self.key_press_type, Key::Num8, "8");
                                ui.selectable_value(&mut self.key_press_type, Key::Num9, "9");

                                ui.selectable_value(&mut self.key_press_type, Key::Space, "Space");
                                ui.selectable_value(
                                    &mut self.key_press_type,
                                    Key::Return,
                                    "Return",
                                );
                                ui.selectable_value(
                                    &mut self.key_press_type,
                                    Key::Backspace,
                                    "Backspace",
                                );
                                ui.selectable_value(
                                    &mut self.key_press_type,
                                    Key::ShiftLeft,
                                    "Shift",
                                );
                                ui.selectable_value(
                                    &mut self.key_press_type,
                                    Key::UpArrow,
                                    "Up Arrow",
                                );
                                ui.selectable_value(
                                    &mut self.key_press_type,
                                    Key::DownArrow,
                                    "Down Arrow",
                                );
                                ui.selectable_value(
                                    &mut self.key_press_type,
                                    Key::LeftArrow,
                                    "Left Arrow",
                                );
                                ui.selectable_value(
                                    &mut self.key_press_type,
                                    Key::RightArrow,
                                    "Right Arrow",
                                );
                            });
                    });
                });
            }

            ui.separator();

            // click interval
            ui.horizontal_wrapped(|ui| {
                ui.label("Click Interval");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("ms");
                    if running {
                        ui.set_enabled(false);
                    };
                    ui.add(
                        egui::TextEdit::singleline(&mut self.ms_str)
                            .desired_width(40.0f32)
                            .hint_text("0"),
                    );
                    ui.label("s");
                    if running {
                        ui.set_enabled(false);
                    };
                    ui.add(
                        egui::TextEdit::singleline(&mut self.s_str)
                            .desired_width(40.0f32)
                            .hint_text("0"),
                    );
                })
            });

            ui.separator();

            // hold
            ui.horizontal_wrapped(|ui| {
                ui.label("Hold");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if running {
                        ui.set_enabled(false);
                    };
                    ui.checkbox(&mut self.hold, "");
                    if self.hold {
                        ui.label("ms");
                        if running {
                            ui.set_enabled(false);
                        };
                        ui.add(
                            egui::TextEdit::singleline(&mut self.hold_ms_str)
                                .desired_width(40.0f32)
                                .hint_text("0"),
                        );
                        ui.label("s");
                        if running {
                            ui.set_enabled(false);
                        };
                        ui.add(
                            egui::TextEdit::singleline(&mut self.hold_s_str)
                                .desired_width(40.0f32)
                                .hint_text("0"),
                        );
                    }
                });
            });

            ui.separator();

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui
                    .add_sized(
                        [120.0f32, 38.0f32],
                        egui::widgets::Button::new(if running {
                            "🖱 STOP (F6)"
                        } else {
                            "🖱 START (F6)"
                        }),
                    )
                    .clicked()
                {
                    if running {
                        self.stop();
                    } else {
                        self.start();
                    }
                };
            });
        });

        ctx.request_repaint();
    }
}
