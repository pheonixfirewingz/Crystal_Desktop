use crate::window::display_manager::DisplayServer;
use std::sync::atomic::{AtomicBool};
use std::sync::{Arc, RwLock};
use glfw::Context;
use libprotocol::ScreenSize;

pub fn start_screen(dm_server: Arc<RwLock<DisplayServer>>, shutdown: Arc<AtomicBool>) {
    #[cfg(debug_assertions)]
    {
    let width = 1920;
    let height = 1080;
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).expect("GLFW init error");
    // Request OpenGL 4.6 Core Profile
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::Decorated(false));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    let (mut window, events) = glfw
        .create_window(
            width as u32,
            height as u32,
            "Debug Screen",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");
    window.make_current();
    window.set_cursor_mode(glfw::CursorMode::Hidden);
    let width = window.get_framebuffer_size().0 as usize;
    let height = window.get_framebuffer_size().1 as usize;
    {
        let mut dm = dm_server
            .write()
            .expect("Failed to get dm_server write lock");
        dm.setup_renderer(width as ScreenSize, height as ScreenSize);
    }
    window.set_all_polling(true);
    let target_frame_time = std::time::Duration::from_secs(1) / 60;
    // Main render loop
    let mut last_mouse_pos = (0.0, 0.0);
    let mut last_scroll_pos = (0.0, 0.0);
    while !window.should_close() && !shutdown.load(std::sync::atomic::Ordering::Relaxed) {
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    shutdown.store(true, std::sync::atomic::Ordering::Relaxed);
                    window.set_should_close(true);
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    let mut dm = dm_server.write().expect("Failed to read display server");
                    // Calculate relative position
                    let relative_mouse_pos = (x - last_mouse_pos.0, y - last_mouse_pos.1);
                    dm.update_mouse_pos(
                        relative_mouse_pos.0 as ScreenSize,
                        relative_mouse_pos.1 as ScreenSize,
                    );
                    // Update the last mouse position
                    last_mouse_pos = (x, y);
                }
                glfw::WindowEvent::Scroll(x, y) => {
                    let mut dm = dm_server.write().expect("Failed to read display server");
                    // Calculate relative position
                    let relative_scroll_pos = (x - last_scroll_pos.0, y - last_scroll_pos.1);
                    dm.update_mouse_wheel_delta(
                        relative_scroll_pos.0 as f32,
                        relative_scroll_pos.1 as f32,
                    );
                    // Update the last scroll position
                    last_scroll_pos = (x, y);
                }
                glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                    let mut dm = dm_server
                        .write()
                        .expect("Failed to acquire write lock on display server");
                    dm.update_button_state(button as u8, action == glfw::Action::Press);
                    if modifiers.contains(glfw::Modifiers::Shift) {
                        println!("Shift modifier is active.");
                    }
                }
                _ => {}
            }
        }
        let start_time = std::time::Instant::now();
        {
            let mut dm = dm_server
                .write()
                .expect("Failed to acquire write lock on display server");
            dm.tick();
        }
        // Swap front and back buffers
        window.swap_buffers();
        let elapsed = start_time.elapsed();
        if elapsed < target_frame_time {
            let sleep_time = target_frame_time - elapsed;
            std::thread::sleep(sleep_time);
        }
    }
    window.set_cursor_mode(glfw::CursorMode::Normal);
    // Cleanup
    shutdown.store(true, std::sync::atomic::Ordering::Relaxed);
}
}