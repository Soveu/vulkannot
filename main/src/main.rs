fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (window, _events) = glfw.create_window(800, 600, "Vulkant", glfw::WindowMode::Windowed).unwrap();
    while !window.should_close() {
        glfw.poll_events();
    }
}
