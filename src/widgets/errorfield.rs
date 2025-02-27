/*

Extended egui::TextEdit::singleline that show a red frame when the `valid` parameter is true. 

*/

use eframe::egui;
use eframe::egui::Widget;

pub struct ErrorField<'a>
{
    value: &'a mut String,
    valid: bool
}

impl<'a> ErrorField<'a>
{
    pub const fn new (value: &'a mut String, valid: bool) -> Self {
        Self {
            value,
            valid
        }
    }

    fn reflect (&mut self, ui: &mut egui::Ui) {
        if !self.valid {
            let visuals = &mut ui.style_mut().visuals;
            visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, visuals.error_fg_color);
            visuals.selection.stroke = egui::Stroke::new(1.0, visuals.error_fg_color);
        }
    }
}

impl Widget for ErrorField<'_>
{
    fn ui (mut self, ui: &mut egui::Ui) -> egui::Response {
        self.reflect(ui);
        egui::TextEdit::singleline(self.value).ui(ui).highlight()
    }
}
