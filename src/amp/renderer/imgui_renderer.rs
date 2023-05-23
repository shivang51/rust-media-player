use std::time::Instant;

use imgui::{MouseCursor, Ui};
use imgui_glfw_support::{GlfwPlatform, HiDpiMode};

pub struct ImGuiRenderer {
    imgui_ctx: Option<imgui::Context>,
    glfw_platform: Option<GlfwPlatform>,
    last_cursor: Option<MouseCursor>,
    imgui_renderer: Option<imgui_opengl_renderer::Renderer>,
    last_frame_time: Instant,
    ui: Option<Ui>,
}

impl Default for ImGuiRenderer {
    fn default() -> Self {
        return ImGuiRenderer {
            imgui_ctx: None,
            glfw_platform: None,
            last_cursor: None,
            imgui_renderer: None,
            last_frame_time: Instant::now(),
            ui: None,
        };
    }
}

impl ImGuiRenderer {
    pub fn init(&mut self, window: &mut glfw::Window) {
        self.imgui_ctx = Some(imgui::Context::create());
        self.imgui_ctx.as_mut().unwrap().set_ini_filename(None);

        self.glfw_platform = Some(GlfwPlatform::init(self.imgui_ctx.as_mut().unwrap()));

        self.glfw_platform.as_mut().unwrap().attach_window(
            self.imgui_ctx.as_mut().unwrap().io_mut(),
            window,
            HiDpiMode::Default,
        );

        unsafe {
            self.glfw_platform
                .as_mut()
                .unwrap()
                .set_clipboard_backend(self.imgui_ctx.as_mut().unwrap(), window);
        }

        self.imgui_renderer = Some(imgui_opengl_renderer::Renderer::new(
            self.imgui_ctx.as_mut().unwrap(),
            |s| window.get_proc_address(s) as _,
        ));

        self.last_frame_time = Instant::now();
    }

    pub fn begin(&mut self, window: &mut glfw::Window) {
        let now = Instant::now();
        self.imgui_ctx
            .as_mut()
            .unwrap()
            .io_mut()
            .update_delta_time(now - self.last_frame_time);

        self.last_frame_time = now;

        self.glfw_platform
            .as_mut()
            .unwrap()
            .prepare_frame(self.imgui_ctx.as_mut().unwrap().io_mut(), window)
            .expect("prepare_frame failed");

        let ui: &mut Ui = self.imgui_ctx.as_mut().unwrap().new_frame();
        unsafe {
            self.ui = Some(std::mem::transmute_copy(ui));
        }
    }

    pub fn end(&mut self, window: &mut glfw::Window) {
        let cursor = self.get_ui_mut().mouse_cursor();
        if self.last_cursor != cursor {
            self.last_cursor = cursor;
            self.glfw_platform
                .as_mut()
                .unwrap()
                .prepare_render(self.ui.as_mut().unwrap(), window);
        }

        self.imgui_renderer
            .as_mut()
            .unwrap()
            .render(self.imgui_ctx.as_mut().unwrap());
    }

    pub fn get_ui_mut(&mut self) -> &mut Ui {
        self.ui.as_mut().unwrap()
    }

    pub fn update(&mut self, event: &glfw::WindowEvent, window: &glfw::Window) {
        self.glfw_platform.as_mut().unwrap().handle_event(
            self.imgui_ctx.as_mut().unwrap().io_mut(),
            window,
            &event,
        );
    }
}

trait GetFrame {
    fn get_frame(&mut self) -> &mut Ui;
}
