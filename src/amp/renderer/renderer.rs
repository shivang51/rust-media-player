use std::default::Default;
use std::time::Instant;

use egui_backend::egui::{vec2, Pos2, Rect};

use super::opengl::opengl;
use egui_backend::{egui, EguiInputState};
use egui_glfw_gl as egui_backend;
use egui_glfw_gl::egui::Color32;

struct EGuiRenderer {
    egui_input_state: Option<EguiInputState>,
    egui_ctx: egui::Context,
    start_time: Instant,
    painter: Option<egui_glfw_gl::Painter>,
    frame_tex_id: egui::TextureId,
}

impl Default for EGuiRenderer {
    fn default() -> Self {
        return EGuiRenderer {
            egui_input_state: None,
            egui_ctx: egui::Context::default(),
            start_time: Instant::now(),
            painter: None,
            frame_tex_id: egui::TextureId::default(),
        };
    }
}

const PIC_WIDTH: i32 = 320;
const PIC_HEIGHT: i32 = 192;

impl EGuiRenderer {
    fn init(&mut self, window: &mut glfw::Window) {
        self.painter = Some(egui_backend::Painter::new(window));
        self.egui_ctx = egui::Context::default();

        let (width, height) = window.get_framebuffer_size();
        let native_pixels_per_point = window.get_content_scale().0;

        let egui_input_state = EguiInputState::new(egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(width as f32, height as f32) / native_pixels_per_point,
            )),
            pixels_per_point: Some(native_pixels_per_point),
            ..Default::default()
        });

        self.egui_input_state = Some(egui_input_state);

        self.start_time = Instant::now();

        let srgba = vec![Color32::DARK_GREEN; (PIC_HEIGHT * PIC_WIDTH) as usize];

        self.frame_tex_id = self.painter.as_mut().unwrap().new_user_texture(
            (PIC_WIDTH as usize, PIC_HEIGHT as usize),
            &srgba,
            egui::TextureFilter::Linear,
        );

        self.painter.as_mut().unwrap().update_user_texture_data(&self.frame_tex_id, &srgba);


    }

    fn begin(&mut self, window: &glfw::Window) {
        let egui_input_state = self.egui_input_state.as_mut().unwrap();
        let native_pixels_per_point = window.get_content_scale().0;

        let (width, height) = window.get_framebuffer_size();

        egui_input_state.input.screen_rect = Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        ));

        egui_input_state.input.time = Some(self.start_time.elapsed().as_secs_f64());
        egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);
        self.egui_ctx.begin_frame(egui_input_state.input.take());
    }

    fn end(&mut self) {
        let egui::FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = self.egui_ctx.end_frame();

        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(
                self.egui_input_state.as_mut().unwrap(),
                platform_output.copied_text,
            );
        }

        let clipped_shapes = self.egui_ctx.tessellate(shapes);
        self.painter.as_mut().unwrap().paint_and_update_textures(
            1.0,
            &clipped_shapes,
            &textures_delta,
        );
    }

    fn draw(&mut self) {
        egui::TopBottomPanel::top("Top")
            .resizable(true)
            .show(&self.egui_ctx, |ui| {
                ui.menu_button("File", |ui| {
                    {
                        let _ = ui.button("test 1");
                    }
                    ui.separator();
                    {
                        let _ = ui.button("test 2");
                    }
                });
            });

        // painter.update_user_texture_data(&plot_tex_id, &srgba);

        egui::Window::new("Window").resizable(true).show(&self.egui_ctx, |ui| {
            if ui.button("Quit").clicked() {
                println!("Clicked!");
            }
            ui.add(egui::Image::new(self.frame_tex_id, vec2(PIC_WIDTH as f32, PIC_HEIGHT as f32)));
        });
    }

    fn update_events(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Size(width, height) => {
                self.egui_input_state.as_mut().unwrap().input.screen_rect = Some(
                    Rect::from_min_size(Pos2::new(0f32, 0f32), vec2(width as f32, height as f32)),
                );
            }
            _ => {}
        }

        egui_backend::handle_event(event, self.egui_input_state.as_mut().unwrap());
    }
}

pub struct Renderer {
    egui_renderer: EGuiRenderer,
}

impl Default for Renderer {
    fn default() -> Self {
        return Renderer {
            egui_renderer: Default::default(),
        };
    }
}

impl Renderer {
    pub fn init(&mut self, window: &mut glfw::Window) {
        opengl::init(window);
        self.egui_renderer.init(window);
    }

    pub fn render(&mut self) {
        self.egui_renderer.draw();
    }

    pub fn begin(&mut self, window: &glfw::Window) {
        self.egui_renderer.begin(window);
    }

    pub fn end(&mut self) {
        self.egui_renderer.end();
    }

    pub fn update(&mut self, event: glfw::WindowEvent) {
        self.egui_renderer.update_events(event);
    }
}
