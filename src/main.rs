/*
Simple template-based parsing and transforming of a text file with values.
*/

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::io::Write;
// use std::thread;

use eframe::egui;
use eframe:: { 
    App, 
    Frame
};

mod widgets;
use crate::widgets::errorfield::ErrorField;
use crate::widgets::switch::Switch;

mod models;
use crate::models::table::Table;
use crate::models::parser::Parser;
use crate::models::parser::Origin;


const WINDOW_SIZE:  egui::Vec2 = egui::Vec2::new(640.0, 480.0);
const ACCENT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 153, 127); // HSL(170,100,30)

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
enum InterfaceMode
{
    Dark,
    Light
}

#[derive(Default, PartialEq)]
enum StateTracker 
{
    Changed,  // When table structure has changed and reset is needed.
    Saving,   // When saving is in progress.
    Dragging, // When export button is being dragged
    #[default] Idle
}

// Extract only message from an Result error by adding a new trait to Result (go Rust!).
trait MessageOnly {
    fn as_message (&self) -> String;
}

impl <T,E> MessageOnly for Result<T,E> where E: ToString {
    fn as_message (&self) -> String {
        match self {
            Ok  (_) => String::new(),
            Err (m) => m.to_string()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Reshaper
{
    source: String,
    target: String,
    ui_size: f32,
    ui_mode: InterfaceMode,

    #[serde(skip)] parser: Parser,
    #[serde(skip)] data: Table,
    #[serde(skip)] path: String,
    #[serde(skip)] source_error: String,
    #[serde(skip)] target_error: String,
    #[serde(skip)] state: StateTracker,
    #[serde(skip)] target_view: bool,
    #[serde(skip)] do_quotes: bool
}

impl Default for Reshaper
{
    fn default() -> Self {
        Self {
            source: String::from("<date> <time>: <systolic>/<diastolic> <pulse>"),
            target: String::from("<date>,<pulse>,<systolic>,<diastolic>"),
            ui_size: 1.2,
            ui_mode: InterfaceMode::Dark,
            parser: Parser::new(),
            data: Table::new(),
            path: String::new(),
            source_error: String::new(),
            target_error: String::new(),
            state: StateTracker::Idle,
            target_view: true,
            do_quotes: false
        }
    }
}

impl Reshaper
{
    fn new (context: &eframe::CreationContext<'_>) -> Self {
        let mut object = if let Some(ps) = context.storage { eframe::get_value(ps, eframe::APP_KEY).unwrap_or_default() } else { Reshaper::default() };
        object.source_error = object.parser.set_source(&object.source).as_message();
        object.target_error = object.parser.set_target(&object.target).as_message();
        object.target_view = true; // Why is this set to false during deserialization?
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
        context.style_mut(|style| {
            style.spacing.item_spacing = egui::Vec2::new(12.0, 8.0);
            style.spacing.button_padding = egui::Vec2::new(8.0, 2.0);
        });
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

    fn create_upper (&mut self, ui: &mut egui::Ui, context: &egui::Context) {
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
            ui.label(egui::RichText::new("SOURCE TEMPLATE").small().weak());
            if ui.add(ErrorField::new(&mut self.source, self.source_error.is_empty())).changed() {
                self.source_error = self.parser.set_source(&self.source).as_message();
                self.target_error = self.parser.set_target(&self.target).as_message(); // Source errors can cause target errors.
                if  self.source_error.is_empty() && self.target_error.is_empty(){
                    self.state = StateTracker::Changed;
                    self.load_file();
                }
            };
            if !self.source_error.is_empty() {
                ui.label(egui::RichText::new(&self.source_error).color(egui::Color32::RED));
            }
            ui.add_space(12.0);
            ui.label(egui::RichText::new("TARGET TEMPLATE").small().weak());
            if ui.add(ErrorField::new(&mut self.target, self.target_error.is_empty())).changed() {
                self.target_error = self.parser.set_target(&self.target).as_message();
                if  self.target_error.is_empty() {
                    self.state = StateTracker::Changed;
                }
            };
            if !self.target_error.is_empty() {
                ui.label(egui::RichText::new(&self.target_error).color(egui::Color32::RED));
            }
        });
        if !self.data.is_empty() {
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if ui.add(Switch::new(self.target_view)).clicked() {
                    self.target_view = !self.target_view;
                };
                ui.label(format!("{} data shown in table.", if self.target_view {"Transformed"} else {"Original"}));
                ui.add_space(12.0);
                let dragger = ui.small_button("\u{e074} Drag to export").interact(egui::Sense::click_and_drag()).highlight();
                if  dragger.drag_started() {
                    self.state = StateTracker::Dragging;
                }
                let outside = !context.screen_rect().contains(ui.input(|i| i.pointer.interact_pos()).unwrap_or_default());
                if  dragger.drag_stopped() {
                    context.set_cursor_icon(egui::CursorIcon::Default);
                    self.state = StateTracker::Idle;
                    if outside {
                        // thread::spawn(|| { // Should be this easy (nogo Rust).
                            self.save_file();
                        // });
                    }
                }
                if self.state == StateTracker::Dragging {
                    context.set_cursor_icon(if outside { egui::CursorIcon::Grabbing } else { egui::CursorIcon::NoDrop });
                }
                ui.checkbox(&mut self.do_quotes, "Write quotation marks");
            });
        }
    }

    fn create_lower (&mut self, ui: &mut egui::Ui, context: &egui::Context) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("TEXT SIZE").small().weak());
                if ui.add(egui::Slider::new(&mut self.ui_size, 1.0..=1.7)).changed() {
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
        });
    }

    fn create_table (&self, ui: &mut egui::Ui, reset: bool) {
        let origin = if self.target_view { Origin::Target } else { Origin::Source };
        ui.style_mut().spacing.item_spacing = egui::Vec2::new(16.0, 0.0);
        let builder = egui_extras::TableBuilder::new(ui)
            .sense(egui::Sense::click())
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .animate_scrolling(false)
            .columns(egui_extras::Column::auto(), self.parser.variables(origin).count()-1)
                .resizable(true)
            .column(egui_extras::Column::remainder());
            if reset {
                builder.reset();
            }
            builder.header(24.0, |mut header| {
                self.parser.variables(origin).for_each(|v| {
                    header.col(|ui| {
                        ui.strong(v);
                    });
                });
            })
            .body(|body| {
                body.rows(20.0, self.data.row_count(), |mut row| {
                    let observation = row.index();
                    for variable in self.parser.positions(origin) {
                        row.col(|ui| {
                            if let Some(text) = self.data.get(observation, *variable) {
                                ui.label(text);
                            }
                        });
                    };
                });
            });

    }

    fn load_file (&mut self) {
        self.data = Table::new();
        if let Ok(file) = std::fs::File::open(&self.path) {
            let reader  = std::io::BufReader::new(file);
            std::io::BufRead::lines(reader).for_each(|row| {
                if let Ok(row) = row {
                    if !row.is_empty() && !row.starts_with('#') { // Treat these lines as comments.
                        self.data.add(row.as_str(), self.parser.split(row.as_str()).unwrap_or_default());
                    }
                }
            });
        }
        // Since TableBuilder needs to be reset.
        self.state = StateTracker::Changed;
    }

    fn save_file (&mut self) {
        self.state = StateTracker::Saving;
        if let Some(desktop) = dirs::desktop_dir() {
            let original = std::path::PathBuf::from(&self.path);
            let mut path = desktop.join(original.file_name().unwrap_or_default());
            path.set_extension("out.csv");
            if let Ok(mut file) = std::fs::File::create(path) {
                for row in 0..self.data.row_count() {
                    if let Some(parts) = self.data.get_parts(row) {
                        if let Ok(mut target) = self.parser.transform(parts, self.do_quotes) {
                            target.push('\n');
                            if file.write_all(target.as_bytes()).is_err() {
                                self.state = StateTracker::Idle;
                                return; // Result should be returned to inform user.
                            }
                        }
                    }
                }
            };
        };
        self.state = StateTracker::Idle;
    }

}

impl App for Reshaper
{
    fn save (&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update (&mut self, context: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("Templates").frame(self.get_frame()).resizable(false).show(context, |ui| {
            self.create_upper(ui, context);
        });
        egui::TopBottomPanel::bottom("Settings").frame(self.get_frame()).resizable(false).show(context, |ui| {
            self.create_lower(ui, context);
        });
        // Must be last for remaining size in the middle to be calculated correctly.
        egui::CentralPanel::default().frame(self.get_frame()).show(context, |ui| {
            if self.state == StateTracker::Saving { // Needs to be in separate thread to be visible?
                egui::CentralPanel::default().frame(self.get_frame()).show(context, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(48.0);
                        ui.spinner();
                        ui.label("Processing data ...");
                    });
                });
                return;
            }; 
            let mut hovered = egui::HoveredFile::default();
            let mut dropped = egui::DroppedFile::default();
            context.input(|input| {
                if !input.raw.hovered_files.is_empty() { hovered = input.raw.hovered_files[0].clone() };
                if !input.raw.dropped_files.is_empty() { dropped = input.raw.dropped_files[0].clone() };
            });
            if hovered.path.is_some() {                
                ui.painter().rect(
                    ui.min_rect(), 
                    0.0, 
                    ui.style().visuals.selection.bg_fill, 
                    egui::Stroke::new(2.0, ACCENT_COLOR), 
                    egui::StrokeKind::Middle
                );
            }
            if dropped.path.is_some() {
                if let Some(path) = &dropped.path {
                    self.path = path.display().to_string();
                    self.load_file();
                };
            }
            if self.data.is_empty() {
                ui.add_sized(ui.available_size(), egui::Label::new(egui::RichText::new("(drop file here)").heading().italics().weak()));
            } else {
                self.create_table(ui, self.state == StateTracker::Changed);
                if self.state == StateTracker::Changed {
                    self.state = StateTracker::Idle; // Need a TableBuilder reset after loading new data.
                }
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
