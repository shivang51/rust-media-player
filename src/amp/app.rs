use egui_glfw_gl::egui::Color32;

use super::renderer::renderer;
use super::window::{self, EventCallback, Window};
use crate::libamp::media_file::MediaFile;

pub struct App {
    renderer: renderer::Renderer,
    main_window: window::Window,
}

impl Default for App {
    fn default() -> Self {
        App {
            renderer: Default::default(),
            main_window: Default::default(),
        }
    }
}

impl App {
    pub fn run(&mut self) {
        self.main_window.init(String::from("AMP"), 800, 500);

        self.renderer.init(self.main_window.get_glfw_hwnd());

        let mut file: MediaFile = Default::default();

        file.open(String::from("F:\\Videos\\1.mp4"));

        unsafe {
            gl::Enable(gl::BLEND);
            gl::ClearColor(0.11, 0.11, 0.11, 1.0);
        }

        let frame = file.decoder.get_video_frame(10.0);

        self.renderer.egui_renderer.frame_data.resize(
            (frame.width * frame.height) as usize,
            Color32::from_rgb(0, 0, 0),
        );

        for i in 0..(frame.width * frame.height) {
            unsafe {
                let px = *(frame.data[0].offset(i as isize));
                self.renderer.egui_renderer.frame_data[i as usize] = Color32::from_rgb(px, px, px)
            }
        }

        self.renderer
            .egui_renderer
            .init_video_frame(frame.width as usize, frame.height as usize);

        while self.main_window.is_open() {
            self.clear();

            self.renderer.begin(&self.main_window.get_glfw_hwnd());

            self.renderer.render();

            self.renderer.end();

            self.update();
        }

        if file.is_open() {
            file.close();
        }
    }

    /// swap buffers and handle the events
    fn update(&mut self) {
        self.main_window.event_callback(&mut self.renderer);
        self.main_window.swap_buffers();
        self.main_window.wait_events();
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
