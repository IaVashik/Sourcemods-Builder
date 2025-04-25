use eframe::egui::{
    self, Align, Context, InnerResponse, Response, TextBuffer, TextEdit, ViewportBuilder,
    ViewportClass, ViewportId, WidgetText,
};
use egui::RichText;

#[allow(dead_code)]
pub trait UiExt {
    fn checkbox_sized(
        &mut self,
        checked: &mut bool,
        text: impl Into<String>,
        size: f32,
    ) -> Response;
    fn label_sized(&mut self, text: impl Into<String>, size: f32) -> Response;
    fn label_size_centered(
        &mut self,
        text: impl Into<String>,
        size: f32,
    ) -> InnerResponse<Response>;
    fn label_on_screen(
        &mut self,
        text: impl Into<WidgetText>,
        spacing_x: f32,
        spacing_y: f32,
    ) -> Response;
    fn singleline_on_screen(&mut self, text: &mut dyn TextBuffer, spacing_x: f32) -> Response;
}

impl UiExt for egui::Ui {
    fn label_sized(&mut self, text: impl Into<String>, size: f32) -> Response {
        self.label(RichText::new(text.into()).size(size))
    }

    fn label_size_centered(
        &mut self,
        text: impl Into<String>,
        size: f32,
    ) -> InnerResponse<Response> {
        self.with_layout(egui::Layout::top_down(Align::Center), |ui| {
            ui.label(RichText::new(text.into()).size(size))
        })
    }

    fn singleline_on_screen(&mut self, text: &mut dyn TextBuffer, spacing_x: f32) -> Response {
        TextEdit::singleline(text)
            .desired_width(self.available_width() - spacing_x)
            .show(self)
            .response
    }

    fn label_on_screen(
        &mut self,
        text: impl Into<WidgetText>,
        spacing_x: f32,
        spacing_y: f32,
    ) -> Response {
        self.add_sized(
            [
                self.available_width() - spacing_x,
                self.spacing().interact_size.y - spacing_y,
            ],
            egui::Label::new(text),
        )
    }

    fn checkbox_sized(
        &mut self,
        checked: &mut bool,
        text: impl Into<String>,
        size: f32,
    ) -> Response {
        let style = self.style_mut();
        let icon_width = style.spacing.icon_width;
        style.spacing.icon_width = size;
        let widget = self.checkbox(checked, RichText::new(text.into()).size(size));
        self.style_mut().spacing.icon_width = icon_width;
        widget
    }
}

pub fn show_viewport_immediate(
    ctx: &Context,
    title: &str,
    size: [f32; 2],
    f: impl FnMut(&eframe::egui::Context, ViewportClass),
) {
    let dpi = ctx.zoom_factor();
    ctx.show_viewport_immediate(
        ViewportId::from_hash_of(title),
        ViewportBuilder::default()
            .with_title(title)
            .with_min_inner_size([size[0] / dpi, size[1] / dpi])
            .with_inner_size(size),
        f,
    );
}
