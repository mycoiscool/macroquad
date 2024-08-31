use crate::{
    math::{Rectangle, Vec2},
    texture::Texture2D,
    ui::{Layout, Ui},
};

pub struct Texture {
    position: Option<Vec2<f32>>,
    w: f32,
    h: f32,
    texture: Texture2D,
}

impl Texture {
    pub fn new(texture: Texture2D) -> Texture {
        Texture {
            position: None,
            w: 100.,
            h: 100.,
            texture,
        }
    }

    pub fn size(self, w: f32, h: f32) -> Self {
        Texture { w, h, ..self }
    }

    pub fn position<P: Into<Option<Vec2<f32>>>>(self, position: P) -> Self {
        let position = position.into();

        Texture { position, ..self }
    }

    pub fn ui(self, ui: &mut Ui) -> bool {
        let context = ui.get_active_window_context();

        let size = Vec2::new(self.w, self.h);

        let pos = context
            .window
            .cursor
            .fit(size, self.position.map_or(Layout::Vertical, Layout::Free));
        context
            .window
            .painter
            .draw_raw_texture(Rectangle::new(pos.x, pos.y, self.w, self.h), &self.texture);

        let rect = Rectangle::new(pos.x, pos.y, size.x as f32, size.y as f32);
        let hovered = rect.contains_point(context.input.mouse_position);

        context.focused && hovered && context.input.click_up
    }
}

impl Ui {
    pub fn texture(&mut self, texture: Texture2D, w: f32, h: f32) -> bool {
        Texture::new(texture).size(w, h).ui(self)
    }
}
