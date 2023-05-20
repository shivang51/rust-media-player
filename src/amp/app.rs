use crate::libamp::decoder::Decoder;
use super::renderer::renderer;
use super::window::{self, EventCallback, Window};

pub struct App {
    renderer: renderer::Renderer,
    main_window: window::Window,
    decoder: Decoder,
}

impl Default for App {
    fn default() -> Self {
        return App {
            renderer: Default::default(),
            main_window: Default::default(),
            decoder: Default::default(),
        };
    }
}

impl App {
    pub fn run(&mut self) {
        self.main_window.init(String::from("AMP"), 800, 500);

        self.renderer.init(self.main_window.get_glfw_hwnd());

        self.decoder.init();
        self.decoder.open(String::from("F:\\Videos\\1.mp4"));

        unsafe {
            gl::Enable(gl::BLEND);
            gl::ClearColor(0.11, 0.11, 0.11, 1.0);
            self.decoder.get_video_stream();
        }

        while self.main_window.is_open() {
            self.clear();

            self.renderer.begin(&self.main_window.get_glfw_hwnd());

            self.renderer.render();

            self.renderer.end();

            self.update();
        }
    }

    /// swap buffers and handle the events
    fn update(&mut self) {
        self.main_window.swap_buffers();
        self.main_window.wait_events();
        self.main_window.event_callback(&mut self.renderer);
    }

    fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}

impl EventCallback for Window {
    fn event_callback(&mut self, renderer: &mut renderer::Renderer) {
        for (_timestamp, event) in glfw::flush_messages(self.events.as_ref().unwrap()) {
            renderer.update(event);
        }
    }
}
