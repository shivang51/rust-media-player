use glfw::{Context, WindowEvent};

use super::renderer::renderer::Renderer;

extern crate glfw;

pub struct Window {
    glfw: Option<glfw::Glfw>,
    window: Option<glfw::Window>,
    pub events: Option<std::sync::mpsc::Receiver<(f64, WindowEvent)>>,
}

impl Default for Window {
    fn default() -> Self {
        return Window {
            glfw: Option::None,
            window: Option::None,
            events: Option::None,
        };
    }
}

impl Window {
    pub fn init(&mut self, name: String, width: u32, height: u32) {
        self.glfw = Some(glfw::init(glfw::FAIL_ON_ERRORS).unwrap());

        let glfw = self.glfw.as_mut().unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(true));
        let (mut window, evts) = glfw
            .create_window(width, height, &name, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_all_polling(true);
        window.make_current();

        self.window = Some(window);
        self.events = Some(evts);
    }

    pub fn swap_buffers(&mut self) {
        self.window.as_mut().unwrap().swap_buffers();
    }

    pub fn wait_events(&mut self) {
        self.glfw.as_mut().unwrap().wait_events();
    }

    pub fn is_open(&mut self) -> bool {
        return !self.window.as_ref().unwrap().should_close();
    }

    pub fn get_glfw_hwnd(&mut self) -> &mut glfw::Window {
        return self.window.as_mut().unwrap();
    }
}

pub trait EventCallback {
    fn event_callback(&mut self, renderer: &mut Renderer);
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window.as_mut().unwrap().set_should_close(true);
    }
}
