/*
 * Blue Engine by Elham Aryanpur
 *
 * Basic GUI example
 *
 * Licensed under Apache-2.0
*/

// Gui is a trait that you'll be using to add your UI
use blue_engine_imgui::{imgui as gui, Gui};

// Basic imports
use blue_engine::{
    header::{Engine, ObjectSettings, WindowDescriptor},
    primitive_shapes::triangle,
};

// Your GUI struct, it'll contain all your UI code
struct MyGUI {
    // Some variables you'd desire
    color: [f32; 4],
}
// Implement the Gui trait
impl Gui for MyGUI {
    // This is the starting point for your UI code, it passes almost all variables of the engine as well
    fn update(
        &mut self,
        _window: &mut blue_engine::Window,
        _renderer: &mut blue_engine::Renderer,
        // We can add underscore to ones we don't use, so they won't emit warnings
        objects: &mut std::collections::HashMap<&'static str, blue_engine::Object>,
        _input: &blue_engine::InputHelper,
        _camera: &mut blue_engine::Camera,
        ui: &gui::Ui,
    ) {
        // Start by creating a UI window inside your app
        gui::Window::new("Control Triangle")
            // Not resizable
            .resizable(false)
            // Let's start adding our UI
            .build(&ui, || {
                // We want a color selector
                gui::ColorEdit::new(
                    // Label for the picker
                    "Pick a color",
                    // Pass in the predefined variable to store the value
                    gui::EditableColor::Float4(&mut self.color),
                )
                .inputs(false)
                .alpha(true)
                .alpha_bar(true)
                .build(&ui);
            });

        // Here we apply the fetched data by changing colors of the triangle
        objects
            .get_mut("triangle")
            .unwrap()
            .set_uniform_color(self.color[0], self.color[1], self.color[2], self.color[3])
            .unwrap();
    }
}

fn main() {
    // Initialize the engine with default settings
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    // Add a triangle to the screen
    triangle("triangle", ObjectSettings::default(), &mut engine).unwrap();

    // Initialize your GUI struct
    let mut my_gui = MyGUI {
        color: [1f32, 1f32, 1f32, 1f32],
    };

    // Start the imgui context
    let gui_context = blue_engine_imgui::ImGUI::init(&engine.window, &mut engine.renderer);

    // Update loop
    engine
        .update_loop(
            move |(renderer, window, objects), (input, camera, (encoder, view), event_fetchers)| {
                // Here we update the imgui context every frame
                event_fetchers[0].update(
                    window,
                    renderer,
                    objects,
                    input,
                    camera,
                    encoder,
                    view,
                    // This is where you add your gui struct
                    &mut my_gui,
                );
            },
            // We add the imgui context here too, so that it fetches input events directly
            vec![gui_context],
        )
        .expect("Error during update loop");
}
