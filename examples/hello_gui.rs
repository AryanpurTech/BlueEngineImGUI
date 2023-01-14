/*
 * Blue Engine by Elham Aryanpur
 *
 * Basic GUI example
 *
 * Licensed under Apache-2.0
*/

// Basic imports
use blue_engine::{
    header::{Engine, ObjectSettings, WindowDescriptor},
    primitive_shapes::triangle,
};

fn main() {
    // Initialize the engine with default settings
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    // Add a triangle to the screen
    triangle(
        "triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .unwrap();
    
    // Start the imgui context
    let gui_context = blue_engine_imgui::ImGUI::new(&engine.window, &mut engine.renderer);
    
    // We add the imgui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.plugins.push(Box::new(gui_context));
    
    // Initialize your variables
    let mut color = [1f32, 1f32, 1f32, 1f32];

    // Update loop
    engine
        .update_loop(move |_, _, objects, _, _, plugins| {
            // select the imgui plugin
            plugins[0]
                // change it back to the original type
                .downcast_mut::<blue_engine_imgui::ImGUI>()
                .unwrap()
                // get the context
                .ui(|ui| {
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
                                &mut color,
                            )
                            .inputs(false)
                            .alpha(true)
                            .alpha_bar(true)
                            .build();
                        });
                });

            // Here we apply the fetched data by changing colors of the triangle
            objects
                .get_mut("triangle")
                .unwrap()
                .set_uniform_color(color[0], color[1], color[2], color[3])
                .unwrap();
        })
        .expect("Error during update loop");
}
