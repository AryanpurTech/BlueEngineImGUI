/* =======================================
 *      DO NOT USE THIS EXAMPLE YET
 * =======================================
 */

use blue_engine::{Engine, WindowDescriptor};

use blue_engine_imgui::{
    imgui as gui,
    imgui::{
        StyleColor, // for colors
        StyleVar,   // for configs
    },
    style_block, // A function that let you style a certain amount of choosing elements
    Gui,         // A trait that lets a struct be used for GUI code
    ImGUI,       // The GUI plugin
    Style,       // lets you choose to change config or color
};

// Your GUI struct, it'll contain all your UI code
struct MyGUI {}
// Implement the Gui trait
impl Gui for MyGUI {
    // This is the starting point for your UI code, it passes almost all variables of the engine as well
    fn update(
        &mut self,
        // We can add underscore to ones we don't use, so they won't emit warnings
        _window: &blue_engine::Window,
        _renderer: &mut blue_engine::Renderer,
        _objects: &mut std::collections::HashMap<&'static str, blue_engine::Object>,
        _camera: &mut blue_engine::Camera,
        _input: &blue_engine::InputHelper,
        _plugin_data_storage: &mut std::collections::HashMap<&'static str, Box<dyn std::any::Any>>,
        ui: &gui::Ui,
    ) {
        // Create a ui window
        ui.window("Styling Buttons").build(|| {
            ui.set_window_font_scale(1.5f32);
            // ========= BLUE ======== //
            style_block(
                vec![Style::Color(
                    StyleColor::Button, // We choose what things to style, e.g. here we choose button
                    [0f32, 0f32, 1f32, 1f32], // And the color data.
                )], // In this vec we add styling
                || {
                    // Here we add elements that we want to style
                    ui.button("Blue Button");

                    style_block(
                        vec![
                            Style::Config(StyleVar::FrameBorderSize(2f32)), // Can apply configs as such
                            Style::Color(StyleColor::Border, [1f32, 1f32, 1f32, 1f32]),
                        ],
                        || {
                            ui.button("Blue Button With Border");
                            // You can also nest them
                            // ========= GREEN ======== //
                            style_block(
                                vec![Style::Color(StyleColor::Button, [0f32, 1f32, 0f32, 1f32])],
                                || {
                                    ui.button("Green Button With Border");
                                },
                                &ui,
                            );
                        },
                        &ui,
                    );
                },
                &ui,
            );
        });
    }
}

fn main() {
    // Initialize the engine with default settings
    let mut engine = Engine::new(WindowDescriptor::default()).unwrap();

    // Start the imgui context
    let gui_context = ImGUI::new(
        &engine.window,
        &mut engine.renderer,
        Box::new(
            MyGUI {}, // Initialize your GUI struct
        ),
    );

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.plugins.push(Box::new(gui_context));

    // update loop
    engine
        .update_loop(|_, _, _, _, _, _| {})
        .expect("Error during update loop");
}
