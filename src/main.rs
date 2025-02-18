/*

Simple template-based parsing and transforming of a text file.

Input format description markup:
    <date> <time>: <systolic>/<diastolic> <pulse>

Possible extension to this could be:
    <date=iso8601date> <time=M|K>: <systolic=u8>/<diastolic=u8> <pulse=u8>
So that expression would be possible in the output.

Output format example:
    <date>;<time>;<systolic>;<diastolic>;<pulse>


Blank means "one or more white space characters".

*/

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use regex::Regex;
use eframe::egui;
use eframe:: { 
    App, 
    Frame
};

mod switch;
mod errorfield;

use switch::Switch;
use errorfield::ErrorField;

const WINDOW_SIZE:  egui::Vec2 = egui::Vec2::new(640.0, 480.0);
const ACCENT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 153, 127); // HSL(170,100,30)

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
enum InterfaceMode
{
    Dark,
    Light
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Reshaper 
{
    source: String,
    target: String,
    data: String,
    ui_size: f32,
    ui_mode: InterfaceMode
}

impl Reshaper 
{
    fn new (context: &eframe::CreationContext<'_>) -> Self {
        let object = if let Some(ps) = context.storage { eframe::get_value(ps, eframe::APP_KEY).unwrap_or_default() } else { Reshaper::default() };
        Self::set_fonts(&context.egui_ctx);
        Self::set_style(&context.egui_ctx, object.ui_mode);
        object
    }

    // Static method, used in new.
    fn set_fonts (context: &egui::Context) {
        let fontname = "Sans Font";
        let mut font = egui::FontDefinitions::default();
        font.font_data.insert(fontname.to_string(), std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/SairaSemiCondensed-Regular.ttf"))));
        if let Some(p) = font.families.get_mut(&egui::FontFamily::Proportional) {
            p.insert(0, fontname.to_string());
            context.set_fonts(font);
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
                visuals.override_text_color = Option::Some(egui::Color32::from_gray(255));
            },
            InterfaceMode::Light => {
                context.set_theme(egui::Theme::Light);
                visuals = egui::Visuals::light();
                visuals.override_text_color = Option::Some(egui::Color32::from_gray(0));
            }
        }
        visuals.widgets.active.bg_fill = ACCENT_COLOR;
        visuals.widgets.noninteractive.bg_fill = ACCENT_COLOR;
        visuals.selection.bg_fill = ACCENT_COLOR.gamma_multiply(0.6);
        visuals.widgets.hovered.bg_fill = ACCENT_COLOR;
        // visuals.widgets.hovered.weak_bg_fill = ACCENT_COLOR.gamma_multiply(0.1);
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
        !self.source.is_empty()
    }

    fn valid_target (&self) -> bool {
        !self.target.is_empty()
    }

}

impl Default for Reshaper 
{
    fn default() -> Self {
        Self {
            source: String::from("<date> <time>: <systolic>/<diastolic> <pulse>"),
            target: String::from("<date>,<time>,<systolic>,<diastolic>,<pulse>"),
            data: String::new(),
            ui_size: 1.3,
            ui_mode: InterfaceMode::Dark
        }
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
        context.style_mut(|writer| writer.spacing.item_spacing = egui::Vec2::new(16.0, 8.0));
        egui::TopBottomPanel::top("").frame(self.get_frame()).exact_height(160.0).resizable(false).show(context, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.label(egui::RichText::new("SOURCE TEMPLATE").small().weak());
                if ui.add(ErrorField::new(&mut self.source, source_is_valid)).lost_focus() {
                    // self.redo_parts(); 
                };
                ui.add_space(12.0);
                ui.label(egui::RichText::new("TARGET TEMPLATE").small().weak());
                if ui.add(ErrorField::new(&mut self.target, target_is_valid)).lost_focus() {
                    // self.redo_parts(); 
                };
            });
        });
        egui::CentralPanel::default().frame(self.get_frame()).show(context, |ui| {
            ui.label(egui::RichText::new("(TABLE COMING ...)").small().weak());
        });
        egui::TopBottomPanel::bottom("").frame(self.get_frame()).exact_height(90.0).resizable(false).show(context, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("TEXT SIZE").small().weak());
                    if ui.add(egui::Slider::new(&mut self.ui_size, 1.0..=2.0)).changed() {
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
