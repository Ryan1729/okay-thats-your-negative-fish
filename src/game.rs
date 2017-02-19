extern crate sdl2;

use platform::Platform;

pub fn go() {
    let mut platform = Platform::new();

    for i in 0..513 {
        platform.draw_coloured_hexagon(i, i, 0xFF000000u32 + i as u32);
    }

    'running: loop {
        let should_quit = platform.quit_on_keypress();

        if should_quit {
            break 'running;
        }

        platform.flip_frame();
        // The rest of the game loop goes here...
    }
}
