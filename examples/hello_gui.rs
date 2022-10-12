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
        _window: &blue_engine::Window,
        _renderer: &mut blue_engine::Renderer,
        // We can add underscore to ones we don't use, so they won't emit warnings
        objects: &mut std::collections::HashMap<&'static str, blue_engine::Object>,
        _camera: &mut blue_engine::Camera,
        _input: &blue_engine::InputHelper,
        _plugin_data_storage: &mut std::collections::HashMap<&'static str, Box<dyn std::any::Any>>,
        ui: &gui::Ui,
    ) {
        // Start by creating a UI window inside your app
        ui.window("Control Triangle")
            // Not resizable
            .resizable(false)
            // Let's start adding our UI
            .build(|| {
                // We want a color selector
                ui.color_edit4_config(
                    // Label for the picker
                    "Pick a color",
                    // Pass in the predefined variable to store the value
                    &mut self.color,
                )
                .inputs(false)
                .alpha(true)
                .alpha_bar(true)
                .build();
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
    let my_gui = MyGUI {
        color: [1f32, 1f32, 1f32, 1f32],
    };

    // Start the imgui context
    let gui_context =
        blue_engine_imgui::ImGUI::new(&engine.window, &mut engine.renderer, Box::new(my_gui));

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.plugins.push(Box::new(gui_context));

    // Update loop
    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}
