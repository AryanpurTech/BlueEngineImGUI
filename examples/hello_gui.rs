/*
 * Blue Engine by Elham Aryanpur
 *
 * Basic GUI example
 *
 * Licensed under Apache-2.0
*/
use blue_engine_imgui::{imgui as gui, Gui};

use blue_engine::{
    header::{Engine, ObjectSettings, WindowDescriptor},
    primitive_shapes::triangle,
};

struct MyGUI {
    color: [f32; 4],
}
impl Gui for MyGUI {
    fn update(
        &mut self,
        _window: &mut blue_engine::Window,
        _renderer: &mut blue_engine::Renderer,
        objects: &mut std::collections::HashMap<&'static str, blue_engine::Object>,
        _input: &blue_engine::InputHelper,
        _camera: &mut blue_engine::Camera,
        ui: &gui::Ui,
    ) {
        gui::Window::new("Control Triangle")
            .resizable(false)
            .build(&ui, || {
                gui::ColorEdit::new("Pick a color", gui::EditableColor::Float4(&mut self.color))
                    .inputs(false)
                    .alpha(true)
                    .alpha_bar(true)
                    .build(&ui);
            });

        objects
            .get_mut("triangle")
            .unwrap()
            .set_uniform_color(self.color[0], self.color[1], self.color[2], self.color[3])
            .unwrap();
    }
}

fn main() {
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    triangle("triangle", ObjectSettings::default(), &mut engine).unwrap();

    let mut my_gui = MyGUI {
        color: [1f32, 1f32, 1f32, 1f32],
    };

    let gui_context = blue_engine_imgui::ImGUI::init(&engine.window, &mut engine.renderer);

    engine
        .update_loop(
            move |(renderer, window, objects), (input, camera, (encoder, view), event_fetchers)| {
                event_fetchers[0].update(
                    window,
                    renderer,
                    objects,
                    input,
                    camera,
                    encoder,
                    view,
                    &mut my_gui,
                );
            },
            vec![gui_context],
        )
        .expect("Error during update loop");
}
