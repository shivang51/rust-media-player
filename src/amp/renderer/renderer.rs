use imgui::TextureId;

use super::{imgui_renderer::ImGuiRenderer, opengl};

pub struct Renderer {
    imgui_renderer: ImGuiRenderer,
    pub frame_texture: opengl::texture::Texture,
}

impl Default for Renderer {
    fn default() -> Self {
        return Renderer {
            imgui_renderer: Default::default(),
            frame_texture: Default::default(),
        };
    }
}

impl Renderer {
    pub fn init(&mut self, window: &mut glfw::Window) {
        opengl::opengl::init(window);
        self.imgui_renderer.init(window);
        self.init_tex();
    }

    fn init_tex(&mut self) {
        self.frame_texture = opengl::texture::Texture::new();
    }

    pub fn render(&mut self, window: &mut glfw::Window) {
        self.imgui_renderer.begin(window);
        self.draw_ui();
        self.imgui_renderer.end(window);
    }

    pub fn update(&mut self, event: &glfw::WindowEvent, window: &glfw::Window) {
        self.imgui_renderer.update(event, window);
    }

    pub fn get_ui_mut(&mut self) -> &mut imgui::Ui {
        self.imgui_renderer.get_ui_mut()
    }

    pub fn get_frame_tex_id(&mut self) -> TextureId {
        self.frame_texture.get_id()
    }
}

pub trait Draw {
    fn draw_ui(&mut self);
}
