use crate::common::{SScreenSize, ScreenSize};
use crate::window::display_manager::DisplayServer;
use glfw::{Action, Context, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

pub fn start_screen(dm_server: Arc<RwLock<DisplayServer>>, shutdown: Arc<AtomicBool>) {
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
    let width = window.get_framebuffer_size().0 as usize;
    let height = window.get_framebuffer_size().1 as usize;
    {
        let mut dm = dm_server
            .write()
            .expect("Failed to get dm_server write lock");
        dm.setup_renderer(width as ScreenSize, height as ScreenSize);
    }
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_mouse_button_polling(true);
    let target_fps = 60; // Target FPS
    let target_frame_time = Duration::from_secs(1) / target_fps;
    // Main render loop
    let mut last_mouse_pos = (0.0, 0.0);
    while !window.should_close() && !shutdown.load(Ordering::Relaxed) {
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                    shutdown.store(true, Ordering::Relaxed);
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    let mut dm = dm_server.write().expect("Failed to read display server");
                    // Calculate relative position
                    let relative_mouse_pos = (x - last_mouse_pos.0, y - last_mouse_pos.1);
                    dm.mouse.add_x(relative_mouse_pos.0 as SScreenSize);
                    dm.mouse.add_y(relative_mouse_pos.1 as SScreenSize);
                    // Update the last mouse position
                    last_mouse_pos = (x, y);
                }
                glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                    let mut dm = dm_server
                        .write()
                        .expect("Failed to acquire write lock on display server");
                    match button {
                        glfw::MouseButtonLeft => {
                            if action == Action::Press {
                                dm.mouse.set_left_button(true);
                            } else if action == Action::Release {
                                dm.mouse.set_left_button(false);
                            }
                        }
                        glfw::MouseButtonRight => {
                            if action == Action::Press {
                                dm.mouse.set_right_button(true);
                            } else if action == Action::Release {
                                dm.mouse.set_right_button(false);
                            }
                        }
                        glfw::MouseButtonMiddle => {
                            if action == Action::Press {
                                dm.mouse.set_middle_button(true);
                            } else if action == Action::Release {
                                dm.mouse.set_middle_button(false);
                            }
                        }
                        _ => {}
                    }
                    // Handle additional modifiers if needed
                    if modifiers.contains(glfw::Modifiers::Shift) {
                        println!("Shift modifier is active.");
                    }
                }
                _ => {}
            }
        }
        let start_time = Instant::now();
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
    // Cleanup
    shutdown.store(true, Ordering::Relaxed);
}
