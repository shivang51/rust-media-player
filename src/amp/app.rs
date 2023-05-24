use crate::libamp::media_file::MediaFile;

use super::renderer::renderer::{Draw, Renderer};
use super::window::{self, EventCallback, Window};
use glfw::WindowEvent;

pub struct App {
    renderer: Renderer,
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

        unsafe {
            gl::Enable(gl::BLEND);
            gl::ClearColor(0.11, 0.11, 0.11, 1.0);
        }

        let mut file = MediaFile::default();
        file.open(String::from("F:\\Videos\\1.mp4"));

        let frame = file.decoder.get_video_frame(10.0);

        let mut data: Vec<u8> = vec![0];
        data.resize((frame.width * frame.height * 3) as usize, 255);

        let mut i: usize = 0;
        while i < ((frame.height * frame.width) as usize) {
            let px = unsafe { *(frame.data[0].offset(i as isize)) };
            data[i] = px;
            data[i + 1] = px;
            data[i + 2] = px;
            i += 3;
        }

        self.renderer
            .frame_texture
            .send_data(frame.width, frame.height, data);

        while self.main_window.is_open() {
            self.clear();

            self.renderer.render(self.main_window.get_glfw_hwnd());

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
    fn event_callback(&mut self, renderer: &mut Renderer) {
        let events: Vec<(f64, WindowEvent)> =
            glfw::flush_messages(self.events.as_ref().unwrap()).collect();
        for (_timestamp, event) in events {
            renderer.update(&event, self.get_glfw_hwnd());
        }
    }
}

impl Draw for Renderer {
    fn draw_ui(&mut self) {
        let id = self.get_frame_tex_id();
        let ui = self.get_ui_mut();

        if let Some(wt) = ui
            .window("Frame")
            .size([100.0, 50.0], imgui::Condition::FirstUseEver)
            .begin()
        {
            ui.button("Hi");
            let pos = ui.window_pos();
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_image_rounded(
                    id,
                    [pos[0] + 0.0, pos[1] + 0.0],
                    [pos[0] + 300.0, pos[1] + 300.0],
                    32.0,
                )
                .build();
            wt.end();
        };

        // let image_size = [300.0, 300.0];
        // let uv_min = [0.0, 0.0];
        // let uv_max = [1.0, 1.0];
        // draw_list.add_image(id, uv_min, uv_max).build();
    }
}
