# Blue Engine ImGUI plugin

This is a plugin that adds [ImGUI](https://github.com/ocornut/imgui) support to the [Blue Engine](https://githb.com/AryanpurTech/BlueEngine).

## Getting started

To get started, create a new struct that will contain your GUI code. For our example:

```rust
struct Counter {
    count: u16
}
```

Then We'll implement the `Gui` trait of the plugin:

```rust
impl Gui for MyGUI {
    // This is the starting point for your UI code, it passes almost all variables of the engine as well
    fn update(
        // for accessing the values
        &mut self,
        _window: &mut blue_engine::Window,
        _renderer: &mut blue_engine::Renderer,
        // We can add underscore to ones we don't use, so they won't emit warnings
        objects: &mut std::collections::HashMap<&'static str, blue_engine::Object>,
        _input: &blue_engine::InputHelper,
        _camera: &mut blue_engine::Camera,
        ui: &blue_engine_imgui::winit::Ui,
    ) {
        /* Your UI code goes here */
    }
}
```

And finally your ImGUI code:

```rust
// Create a new imgui window to contain the UI
gui::Window::new("Counter Window").build(ui, || {
    // Add a text to display the counter
    ui.text(format!("The count is at: {}", self.counter));\
    
    // + 1 per click
    if ui.button("Add 1") {
        self.counter += 1;
    }
});
```

A few more steps are left to be done to have the plugin working. First we need to initialize the plugin before update loop:

```rust
let gui_context = blue_engine_imgui::ImGUI::new(&engine.window, &mut engine.renderer);
```

This will essentially initializes the imgui and create things required to run the plugin. Next step is inside the update loop, we'll need to update the plugin with encoder so that it can create a renderpass for displaying the UI. Here we will pass our UI struct as well:

```rust
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
```

> The `event_fetchers` is the last component in second list in update loop: [ [ _, _, _ ] , [ _, _, _, event_fetcher ] ]

And finally, last step is to add the context too the list of event fetchers, so that it'll get access to all the events immediately and fetch all inputs.

## Style Block

*The guide will come soon, it's cool I promise!*

## Examples

Check the [examples](https://github.com/AryanpurTech/BlueEngineImGUI/tree/master/examples) folder for potential UIs and as template for your new projects.

## Dependency justification

* `blue_engine`: Used obiously for exporting some components and struct declrations required to design the API
* `imgui-wgpu`: Used to assist in applying ImGUI to wgpu graphics backend. Which is same graphics backend used in Blue Engine.
* `imgui-winit-support`: Support for Winit windowing. Which is same windowing system used in Blue Engine.
* `imgui`: The imgui itself, required to obtain components and declrations for api design.
