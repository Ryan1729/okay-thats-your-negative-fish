extern crate sdl2;

use platform::Platform;

pub fn go() {
    let mut platform = Platform::new();

    for i in 0..500 {
        platform.draw_hexagon(i, i);
    }

    'running: loop {
        let should_quit = platform.quit_on_keypress();

        if should_quit {
            break 'running;
        }

        // The rest of the game loop goes here...
    }
}
