use std::time::Instant;

#[allow(unused)]
pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
    warn_span,
};

use glium::{
    glutin::{
        self,
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
    },
    Display, Surface,
};
use imgui::{Context as ImguiContext, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer as ImguiRenderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

//use super::camera::{Camera, CAMERA_SPEED, ZOOM_FACTOR};
use la::{Cell, Universe};

pub const CELL_SIZE: u32 = 10;
pub const CELL_PADDING: u32 = 2;

pub struct Renderer {
    imgui_context: ImguiContext,
    platform: WinitPlatform,
    imgui_renderer: ImguiRenderer,
    font_size: f32,
    //camera: Camera,
}

impl Renderer {
    pub fn init(display: &Display) -> Self {
        let mut imgui_context = ImguiContext::create();
        imgui_context.set_ini_filename(None);

        //if let Some(backend) = clipboard::init() {
        //imgui_context.set_clipboard_backend(Box::new(backend));
        //} else {
        //error!("Failed to initalize clipboard");
        //}

        let mut platform = WinitPlatform::init(&mut imgui_context);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui_context.io_mut(), &window, HiDpiMode::Rounded);
        }

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui_context.fonts().add_font(&[
            FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: font_size,
                    ..FontConfig::default()
                }),
            },
            FontSource::TtfData {
                data: include_bytes!("../../../res/mplus-1p-regular.ttf"),
                size_pixels: font_size,
                config: Some(FontConfig {
                    rasterizer_multiply: 1.75,
                    glyph_ranges: FontGlyphRanges::japanese(),
                    ..FontConfig::default()
                }),
            },
        ]);

        imgui_context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let imgui_renderer = ImguiRenderer::init(&mut imgui_context, display)
            .expect("Failed to create ImguiRenderer");

        //let camera = Camera::new();

        Self {
            imgui_context,
            platform,
            imgui_renderer,
            font_size,
            //camera,
        }
    }

    pub fn handle_event(&mut self, event: Event<()>, display: &Display) {
        match event {
            //Event::WindowEvent {
            //event: WindowEvent::ReceivedCharacter('w'),
            //..
            //} => self.camera.position.1 -= CAMERA_SPEED,
            //Event::WindowEvent {
            //event: WindowEvent::ReceivedCharacter('s'),
            //..
            //} => self.camera.position.1 += CAMERA_SPEED,
            //Event::WindowEvent {
            //event: WindowEvent::ReceivedCharacter('a'),
            //..
            //} => self.camera.position.0 -= CAMERA_SPEED,
            //Event::WindowEvent {
            //event: WindowEvent::ReceivedCharacter('d'),
            //..
            //} => self.camera.position.0 += CAMERA_SPEED,
            //Event::WindowEvent {
            //event: WindowEvent::ReceivedCharacter('q'),
            //..
            //} => self.camera.zoom_level /= ZOOM_FACTOR,
            //Event::WindowEvent {
            //event: WindowEvent::ReceivedCharacter('e'),
            //..
            //} => self.camera.zoom_level *= ZOOM_FACTOR,
            event => {
                self.platform.handle_event(
                    self.imgui_context.io_mut(),
                    display.gl_window().window(),
                    &event,
                );
            }
        }
    }

    pub fn update(&mut self) {
        // actually NewEvents
        //self.last_frame = self.imgui_context.io_mut().update_delta_time(last_frame)
        let mut last_frame = Instant::now();
    }

    pub fn render(&mut self, universe: &Universe, display: &Display) {
        let mut target = display.draw();
        target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
        self.platform
            .prepare_render(&self.imgui_context.frame(), display.gl_window().window());
        let draw_data = self.imgui_context.frame().render();
        self.imgui_renderer
            .render(&mut target, draw_data)
            .expect("Rendering failed");
        target.finish().expect("Faield to swap buffers");

        self.platform
            .prepare_frame(self.imgui_context.io_mut(), &display.gl_window().window())
            .expect("Failed to prepare frame");

        // UI

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // Will draw channel 0 first, then channel 1, whatever the order of
        // the calls in the code.
        //
        // Here, we draw a red line on channel 1 then a white circle on
        // channel 0. As a result, the red line will always appear on top of
        // the white circle.

        //self.imgui_context
        //.frame()
        //.get_window_draw_list()
        //.channels_split(2, |channels| {
        //const RADIUS: f32 = 100.0;
        //let canvas_pos = self.imgui_context.frame().cursor_screen_pos();
        //channels.set_current(1);
        //self.imgui_context
        //.frame()
        //.get_window_draw_list()
        //.add_line(
        //canvas_pos,
        //[canvas_pos[0] + RADIUS, canvas_pos[1] + RADIUS],
        //RED,
        //)
        //.thickness(5.0)
        //.build();

        //channels.set_current(0);
        //let center = [canvas_pos[0] + RADIUS, canvas_pos[1] + RADIUS];
        //self.imgui_context
        //.frame()
        //.get_window_draw_list()
        //.add_circle(center, RADIUS, WHITE)
        //.thickness(10.0)
        //.num_segments(50)
        //.build();
        //});

        // CELLS

        //let canvas = &mut self.canvas;

        //canvas.set_draw_color(Color::BLACK);
        //canvas.clear();
        //canvas.set_draw_color(Color::GREEN);

        // calculate range in which we have to Universe::get_cell
        //let x_range = self.camera.x_range(canvas.window());
        //let y_range = self.camera.y_range(canvas.window());

        //for y in y_range {
        //for x in x_range.clone() {
        //let alive = match universe.get_cell((x, y)) {
        //Cell::Dead => false,
        //Cell::Alive => true,
        //};

        //if alive {
        //let rect = self.camera.project(canvas.window(), (x, y));
        //canvas.fill_rect(rect).unwrap();
        //}
        //}
        //}
    }
}
