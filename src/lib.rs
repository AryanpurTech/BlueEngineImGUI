use blue_engine::{
    Camera, InputHelper, Object, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    Renderer, TextureView, UpdateEvents, Window as Win,
};

pub use imgui;
use imgui::{FontSource, Ui};

pub trait Gui {
    fn update(
        &mut self,
        _window: &mut Win,
        _renderer: &mut Renderer,
        _objects: &mut std::collections::HashMap<&'static str, Object>,
        _input: &blue_engine::InputHelper,
        _camera: &mut Camera,
        ui: &Ui,
    );
}

pub struct ImGUI {
    pub context: imgui::Context,
    pub platform: imgui_winit_support::WinitPlatform,
    pub renderer: imgui_wgpu::Renderer,
    pub last_frame: std::time::Instant,
}

impl ImGUI {
    pub fn init(window: &Win, renderer: &mut Renderer) -> Self {
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);

        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );

        imgui.set_ini_filename(None);

        let hidpi_factor = window.scale_factor();

        imgui_redesign(&mut imgui, hidpi_factor);

        let imgui_renderer = imgui_wgpu::Renderer::new(
            &mut imgui,
            &renderer.device,
            &renderer.queue,
            imgui_wgpu::RendererConfig {
                #[cfg(not(feature = "android"))]
                texture_format: renderer
                    .surface
                    .as_ref()
                    .unwrap()
                    .get_supported_formats(&renderer.adapter)[0],
                ..Default::default()
            },
        );

        let last_frame = std::time::Instant::now();

        Self {
            context: imgui,
            platform,
            renderer: imgui_renderer,
            last_frame,
        }
    }

    /*
    (
                    &mut Renderer,
                    &mut Window,
                    &mut std::collections::HashMap<&'static str, Object>,
                ),
                // Utils
                (
                    &winit_input_helper::WinitInputHelper,
                    &mut Camera,
                    (&mut wgpu::CommandEncoder, &wgpu::TextureView),
                    &mut Vec<T>,
                ) */

    pub fn update<T: Gui + 'static>(
        &mut self,
        window: &mut Win,
        renderer: &mut Renderer,
        objects: &mut std::collections::HashMap<&'static str, Object>,
        input: &InputHelper,
        camera: &mut Camera,
        encoder: &mut blue_engine::CommandEncoder,
        view: &TextureView,
        gui_struct: &mut T,
    ) {
        let now = std::time::Instant::now();
        self.context
            .io_mut()
            .update_delta_time(now - self.last_frame);
        self.last_frame = now;

        self.platform
            .prepare_frame(self.context.io_mut(), &window)
            .expect("Failed to prepare frame");

        let ui = self.context.frame();

        gui_struct.update(window, renderer, objects, input, camera, &ui);
        //gui(&ui, window, renderer, objects);

        let draw_data = ui.render();

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: blue_engine::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        self.renderer
            .render(
                draw_data,
                &renderer.queue,
                &renderer.device,
                &mut render_pass,
            )
            .unwrap();
    }
}

impl UpdateEvents for ImGUI {
    fn update_events<T>(
        &mut self,
        _renderer: &mut Renderer,
        _window: &Win,
        _objects: &mut std::collections::HashMap<&'static str, blue_engine::Object>,
        _events: &blue_engine::Event<T>,
        _camera: &mut blue_engine::Camera,
    ) {
        if _renderer.surface.is_some() {
            self.platform
                .handle_event(self.context.io_mut(), &_window, &_events);
        }
    }
}

// ===============================================================================================

fn imgui_redesign(imgui: &mut imgui::Context, hidpi_factor: f64) {
    let font_size = (13.0 * hidpi_factor) as f32;

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../resources/JetBrainsMono-Medium.ttf"),
        size_pixels: 20f32,
        config: Some(imgui::FontConfig {
            name: Some("JetBrainsMono".to_string()),
            ..Default::default()
        }),
    }]);

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    imgui.set_renderer_name(Some("Blue Engine".to_string()));

    let mut style = imgui.style_mut();

    // Theme by https://github.com/ocornut/imgui/issues/707#issuecomment-917151020
    // Colors
    style.colors[imgui::sys::ImGuiCol_Text as usize] = [1f32, 1f32, 1f32, 1f32];
    style.colors[imgui::sys::ImGuiCol_TextDisabled as usize] = [0.5f32, 0.5f32, 0.5f32, 1f32];
    style.colors[imgui::sys::ImGuiCol_WindowBg as usize] = [0.1f32, 0.1f32, 0.1f32, 1f32];
    style.colors[imgui::sys::ImGuiCol_PopupBg as usize] = [0.19f32, 0.19f32, 0.19f32, 0.92f32];
    style.colors[imgui::sys::ImGuiCol_Border as usize] = [0.19f32, 0.19f32, 0.19f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_BorderShadow as usize] = [0.00f32, 0.00f32, 0.00f32, 0.24f32];
    style.colors[imgui::sys::ImGuiCol_FrameBg as usize] = [0.05f32, 0.05f32, 0.05f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_FrameBgHovered as usize] =
        [0.19f32, 0.19f32, 0.19f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_FrameBgActive as usize] =
        [0.20f32, 0.22f32, 0.23f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_TitleBg as usize] = [0.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_TitleBgActive as usize] =
        [0.06f32, 0.06f32, 0.06f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_TitleBgCollapsed as usize] =
        [0.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_MenuBarBg as usize] = [0.14f32, 0.14f32, 0.14f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_ScrollbarBg as usize] = [0.05f32, 0.05f32, 0.05f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_ScrollbarGrab as usize] =
        [0.34f32, 0.34f32, 0.34f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_ScrollbarGrabHovered as usize] =
        [0.40f32, 0.40f32, 0.40f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_ScrollbarGrabActive as usize] =
        [0.56f32, 0.56f32, 0.56f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_CheckMark as usize] = [0.33f32, 0.67f32, 0.86f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_SliderGrab as usize] = [0.34f32, 0.34f32, 0.34f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_SliderGrabActive as usize] =
        [0.56f32, 0.56f32, 0.56f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_Button as usize] = [0.05f32, 0.05f32, 0.05f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_ButtonHovered as usize] =
        [0.19f32, 0.19f32, 0.19f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_ButtonActive as usize] = [0.20f32, 0.22f32, 0.23f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_Header as usize] = [0.00f32, 0.00f32, 0.00f32, 0.52f32];
    style.colors[imgui::sys::ImGuiCol_HeaderHovered as usize] =
        [0.00f32, 0.00f32, 0.00f32, 0.36f32];
    style.colors[imgui::sys::ImGuiCol_HeaderActive as usize] = [0.20f32, 0.22f32, 0.23f32, 0.33f32];
    style.colors[imgui::sys::ImGuiCol_Separator as usize] = [0.28f32, 0.28f32, 0.28f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_SeparatorHovered as usize] =
        [0.44f32, 0.44f32, 0.44f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_SeparatorActive as usize] =
        [0.40f32, 0.44f32, 0.47f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_ResizeGrip as usize] = [0.28f32, 0.28f32, 0.28f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_ResizeGripHovered as usize] =
        [0.44f32, 0.44f32, 0.44f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_ResizeGripActive as usize] =
        [0.40f32, 0.44f32, 0.47f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_Tab as usize] = [0.00f32, 0.00f32, 0.00f32, 0.52f32];
    style.colors[imgui::sys::ImGuiCol_TabHovered as usize] = [0.14f32, 0.14f32, 0.14f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_TabActive as usize] = [0.20f32, 0.20f32, 0.20f32, 0.36f32];
    style.colors[imgui::sys::ImGuiCol_TabUnfocused as usize] = [0.00f32, 0.00f32, 0.00f32, 0.52f32];
    style.colors[imgui::sys::ImGuiCol_TabUnfocusedActive as usize] =
        [0.14f32, 0.14f32, 0.14f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_PlotLines as usize] = [1.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_PlotLinesHovered as usize] =
        [1.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_PlotHistogram as usize] =
        [1.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_PlotHistogramHovered as usize] =
        [1.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_TableHeaderBg as usize] =
        [0.00f32, 0.00f32, 0.00f32, 0.52f32];
    style.colors[imgui::sys::ImGuiCol_TableBorderStrong as usize] =
        [0.00f32, 0.00f32, 0.00f32, 0.52f32];
    style.colors[imgui::sys::ImGuiCol_TableBorderLight as usize] =
        [0.28f32, 0.28f32, 0.28f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_TableRowBg as usize] = [0.00f32, 0.00f32, 0.00f32, 0.00f32];
    style.colors[imgui::sys::ImGuiCol_TableRowBgAlt as usize] =
        [1.00f32, 1.00f32, 1.00f32, 0.06f32];
    style.colors[imgui::sys::ImGuiCol_TextSelectedBg as usize] =
        [0.20f32, 0.22f32, 0.23f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_DragDropTarget as usize] =
        [0.33f32, 0.67f32, 0.86f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_NavHighlight as usize] = [1.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_NavWindowingHighlight as usize] =
        [1.00f32, 0.00f32, 0.00f32, 0.70f32];
    style.colors[imgui::sys::ImGuiCol_NavWindowingDimBg as usize] =
        [1.00f32, 0.00f32, 0.00f32, 0.20f32];
    style.colors[imgui::sys::ImGuiCol_ModalWindowDimBg as usize] =
        [1.00f32, 0.00f32, 0.00f32, 0.35f32];

    // Configs
    style.window_padding = [8f32, 8f32];
    style.frame_padding = [5f32, 2f32];
    style.cell_padding = [6f32, 6f32];
    style.item_spacing = [6f32, 6f32];
    style.item_inner_spacing = [6f32, 6f32];
    style.touch_extra_padding = [0f32, 0f32];
    style.indent_spacing = 25f32;
    style.scrollbar_size = 15f32;
    style.grab_min_size = 10f32;
    style.window_border_size = 1f32;
    style.child_border_size = 1f32;
    style.popup_border_size = 1f32;
    style.frame_border_size = 1f32;
    style.tab_border_size = 1f32;
    style.window_rounding = 7f32;
    style.child_rounding = 4f32;
    style.frame_rounding = 3f32;
    style.popup_rounding = 4f32;
    style.scrollbar_rounding = 9f32;
    style.grab_rounding = 3f32;
    style.log_slider_deadzone = 4f32;
    style.tab_rounding = 4f32;
}
