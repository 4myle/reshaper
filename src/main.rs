/*

Simple template-based parsing and transforming of a text file.

Input format description markup:
    `<date> <time>: <systolic>/<diastolic> <pulse>`

Possible extension to this could be:
    `<date=iso8601date> <time=M|K>: <systolic=u8>/<diastolic=u8> <pulse=u8>`
So that expression would be possible in the output.

Output format example:
    `<date>;<time>;<systolic>;<diastolic>;<pulse>`

Blank means "one or more white space characters".

*/

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use regex::Regex;
use eframe::egui;
use eframe:: { 
    App, 
    Frame
};

mod widgets;
use crate::widgets::errorfield::ErrorField;
use crate::widgets::switch::Switch;

mod models;
use crate::models::row::Row;
// use crate::models::table::Table;


const WINDOW_SIZE:  egui::Vec2 = egui::Vec2::new(640.0, 480.0);
const ACCENT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 153, 127); // HSL(170,100,30)

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
enum InterfaceMode
{
    Dark,
    Light
}

// Implement data as Vec<Row>, where Row is struct with data: String and part: Vec<&str> (or [&str]?).

#[derive(serde::Deserialize, serde::Serialize)]
struct Reshaper
{
    source: String,
    target: String,
    ui_size: f32,
    ui_mode: InterfaceMode,

    #[serde(skip)]
    data: Vec<Row>
    // data: Table
}

impl Default for Reshaper
{
    fn default() -> Self {
        Self {
            source: String::from("<date> <time>: <systolic>/<diastolic> <pulse>"),
            target: String::from("<date>,<pulse>,<systolic>,<diastolic>"),
            ui_size: 1.2,
            ui_mode: InterfaceMode::Dark,
            data: Vec::new()
            // data: Table::new()
        }
    }
}

impl Reshaper
{
    fn new (context: &eframe::CreationContext<'_>) -> Self {
        let object = if let Some(ps) = context.storage { eframe::get_value(ps, eframe::APP_KEY).unwrap_or_default() } else { Reshaper::default() };
        Self::set_fonts(&context.egui_ctx);
        Self::set_style(&context.egui_ctx, object.ui_mode);
        context.egui_ctx.set_zoom_factor(object.ui_size);
        object
    }

    // Static method, used in new.
    fn set_fonts (context: &egui::Context) {
        let textfont = "Sans Font";
        let iconfont = "Icons";
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            .insert(
                textfont.to_string(), 
                std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/SairaSemiCondensed-Regular.ttf")))
            );
        fonts
            .font_data
            .insert(
                iconfont.to_string(), 
                std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/MaterialIconsOutlined-Regular.otf"))
                    .tweak(egui::FontTweak { 
                        scale: 1.1, 
                        ..Default::default() 
                    }
            )));
        if let Some(p) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            p.insert(0, textfont.to_string());
            p.insert(1, iconfont.to_string());
            context.set_fonts(fonts);
        };
    }

    // Static method, used in new.
    fn set_style (context: &egui::Context, mode: InterfaceMode) {
        // Use context.style().visuals.dark_mode instead of own tracking through InterfaceMode?
        let mut visuals: egui::Visuals;
        match mode {
            InterfaceMode::Dark  => {
                context.set_theme(egui::Theme::Dark);
                visuals = egui::Visuals::dark();
                visuals.override_text_color = Some(egui::Color32::WHITE);
            },
            InterfaceMode::Light => {
                context.set_theme(egui::Theme::Light);
                visuals = egui::Visuals::light();
                visuals.override_text_color = Some(egui::Color32::BLACK);
            }
        }
        visuals.widgets.active.bg_fill = ACCENT_COLOR;
        visuals.widgets.noninteractive.bg_fill = ACCENT_COLOR;
        visuals.widgets.hovered.bg_fill = ACCENT_COLOR;
        visuals.selection.stroke.color  = ACCENT_COLOR; 
        visuals.selection.bg_fill = ACCENT_COLOR.gamma_multiply(0.35);
        visuals.slider_trailing_fill = true;
        context.set_visuals(visuals);
    }
    
    fn get_frame (&mut self) -> egui::Frame {
        let color = match self.ui_mode {
            InterfaceMode::Dark  => egui::Color32::from_rgb( 15,  20,  15),
            InterfaceMode::Light => egui::Color32::from_rgb(245, 250, 245)
        };
        egui::Frame {
            inner_margin: egui::Margin::same(24),
            fill: color,
            ..Default::default()
        }
    }

    fn valid_source (&self) -> bool {
        let _re = Regex::new(r"Hello (?<name>\w+)!");
        !self.source.is_empty()
    }

    fn valid_target (&self) -> bool {
        !self.target.is_empty()
    }

    fn create_table (&self, ui: &mut egui::Ui, reset: bool) {
        ui.style_mut().spacing.item_spacing = egui::Vec2::new(16.0, 0.0);
        let builder = egui_extras::TableBuilder::new(ui)
            .sense(egui::Sense::click())
            .cell_layout(egui::Layout::left_to_right(egui::Align::TOP))
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::remainder());
            if reset {
                builder.reset();
            }
            builder.header(24.0, |mut header| {
                header.col(|ui| {
                    ui.horizontal(|ui| {
                        ui.strong("date");
                        ui.button("\u{e316}").clicked();
                    });
                });
                header.col(|ui| {
                    ui.strong("pulse");
                });
                header.col(|ui| {
                    ui.strong("systolic");
                });
                header.col(|ui| {
                    ui.strong("diastolic");
                });
            })
            .body(|body| {
                body.rows(20.0, 100, |mut row| {
                    row.set_selected(row.index() == 7);
                    row.col(|ui| {
                        // if self.data[0].is_empty() { ui.label("2025-02-22") } else { ui.label("2025-02-21") }; // To supress warning.
                        if !self.data.is_empty() {
                            if let Some(text) = self.data[0].get(0) {
                                ui.label(text);
                            }
                            // if let Some(text) = self.data.get(0).and_then(|r| r.get(0)) { //TODO: implement properly.
                            //     ui.label(text);
                            // }
                        };
                    });
                    row.col(|ui| {
                        ui.label("70");
                    });
                    row.col(|ui| {
                        ui.label("128");
                    });
                    row.col(|ui| {
                        ui.label("75");
                    });
                });
            });

    }

}

impl App for Reshaper
{
    fn save (&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update (&mut self, context: &egui::Context, _frame: &mut Frame) {
        let source_is_valid = self.valid_source();
        let target_is_valid = self.valid_target();
        context.style_mut(|writer| {
            writer.spacing.item_spacing = egui::Vec2::new(12.0, 8.0);
            writer.spacing.button_padding = egui::Vec2::new(2.0, 0.0);
        });
        egui::TopBottomPanel::top("Templates").frame(self.get_frame()).resizable(false).show(context, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.label(egui::RichText::new("SOURCE TEMPLATE").small().weak());
                if ui.add(ErrorField::new(&mut self.source, source_is_valid)).lost_focus() {
                };
                ui.add_space(12.0);
                ui.label(egui::RichText::new("TARGET TEMPLATE").small().weak());
                if ui.add(ErrorField::new(&mut self.target, target_is_valid)).lost_focus() {
                };
            });
        });
        egui::TopBottomPanel::bottom("Settings").frame(self.get_frame()).resizable(false).show(context, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("TEXT SIZE").small().weak());
                    if ui.add(egui::Slider::new(&mut self.ui_size, 0.90..=1.75)).changed() {
                        context.set_zoom_factor(self.ui_size);
                    };
                });
                ui.add_space(24.0);
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("DARK MODE").small().weak());
                    if ui.add(Switch::new(InterfaceMode::Dark == self.ui_mode)).clicked() {
                        match self.ui_mode {
                            InterfaceMode::Dark  => { 
                                self.ui_mode = InterfaceMode::Light;
                                Self::set_style(ui.ctx(), InterfaceMode::Light);
                            },
                            InterfaceMode::Light => { 
                                self.ui_mode = InterfaceMode::Dark;
                                Self::set_style(ui.ctx(), InterfaceMode::Dark);
                            }
                        }
                    };
                });
                ui.add_space(48.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                    ui.style_mut().spacing.button_padding = egui::Vec2::new(8.0, 2.0);
                    if ui.button("\u{e171}  Hide table").clicked() {
                        //TODO: temporary test.
                        // self.data = Table::new();
                        self.data = Vec::new();
                    };
                    if ui.button("\u{eaf3}  Show table").clicked() {
                        //TODO: temporary test.
                        self.data.push(Row::new(String::from("\"2024-10-25\" M: 131/79 63")));
                        self.data[0].add(1,11);
                        // if let Some(r) = self.data.add(String::from("\"2024-10-25\" M: 131/79 63")) {
                        //     r.add(1, 11);
                        // }
                    };
                });
            });
        });
        // Must be last for remaining size to be calculated correctly.
        egui::CentralPanel::default().frame(self.get_frame()).show(context, |ui| { 
            if self.data.is_empty() {
                ui.add_sized(ui.available_size(), egui::Label::new(egui::RichText::new("(drop file here)").heading().italics().weak()));
            } else {
                self.create_table(ui, false);
            }
        });
    }

}

fn main() -> eframe::Result {
    eframe::run_native(
        "Reshaper", 
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_resizable(true)
                .with_maximize_button(true)
                .with_minimize_button(true)
                .with_min_inner_size(WINDOW_SIZE)
                .with_inner_size(WINDOW_SIZE)
                .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../assets/Reshaper.png")[..]).unwrap_or_default()),
            ..Default::default()
        },
        Box::new(|context| {
            Ok(Box::new(Reshaper::new(context)))
        })
    )
}
