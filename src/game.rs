extern crate sdl2;

use platform::Platform;

pub fn go() {
    let mut platform = Platform::new();

    platform.draw_hexagon(10, 10);

    'running: loop {
        let should_quit = platform.quit_on_keypress();

        if should_quit {
            break 'running;
        }

        // The rest of the game loop goes here...
    }
}
