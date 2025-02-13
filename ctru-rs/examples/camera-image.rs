//! Camera image example.
//!
//! This example demonstrates how to use the built-in cameras to take a picture and display it to the screen.

use ctru::prelude::*;
use ctru::services::cam::{Cam, Camera, OutputFormat, ShutterSound, ViewSize};
use ctru::services::gfx::{Flush, Screen, Swap};
use ctru::services::gspgpu::FramebufferFormat;

use std::time::Duration;

const WIDTH: usize = 400;
const HEIGHT: usize = 240;

// The screen size is the width and height multiplied by 2 (RGB565 store pixels in 2 bytes).
const BUF_SIZE: usize = WIDTH * HEIGHT * 2;

const WAIT_TIMEOUT: Duration = Duration::from_millis(300);

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::new().expect("Failed to initialize Apt service.");
    let mut hid = Hid::new().expect("Failed to initialize Hid service.");
    let gfx = Gfx::new().expect("Failed to initialize GFX service.");

    let mut top_screen = gfx.top_screen.borrow_mut();
    top_screen.set_double_buffering(true);
    top_screen.set_framebuffer_format(FramebufferFormat::Rgb565);

    let _console = Console::new(gfx.bottom_screen.borrow_mut());

    println!("Initializing camera");

    let mut cam = Cam::new().expect("Failed to initialize CAM service.");

    // Camera setup.
    {
        let camera = &mut cam.outer_right_cam;

        camera
            .set_view_size(ViewSize::TopLCD)
            .expect("Failed to set camera size");
        camera
            .set_output_format(OutputFormat::Rgb565)
            .expect("Failed to set camera output format");
        camera
            .set_noise_filter(true)
            .expect("Failed to enable noise filter");
        camera
            .set_auto_exposure(true)
            .expect("Failed to enable auto exposure");
        camera
            .set_auto_white_balance(true)
            .expect("Failed to enable auto white balance");
        camera
            .set_trimming(false)
            .expect("Failed to disable trimming");
    }

    let mut buf = vec![0u8; BUF_SIZE];

    println!("\nPress R to take a new picture");
    println!("Press Start to exit");

    while apt.main_loop() {
        hid.scan_input();
        let keys_down = hid.keys_down();

        if keys_down.contains(KeyPad::START) {
            break;
        }

        // If the user presses the R button.
        if keys_down.contains(KeyPad::R) {
            println!("Capturing new image");

            let camera = &mut cam.outer_right_cam;

            // Take a picture and write it to the buffer.
            camera
                .take_picture(
                    &mut buf,
                    WIDTH.try_into().unwrap(),
                    HEIGHT.try_into().unwrap(),
                    WAIT_TIMEOUT,
                )
                .expect("Failed to take picture");

            // Play the normal shutter sound.
            cam.play_shutter_sound(ShutterSound::Normal)
                .expect("Failed to play shutter sound");

            // Rotate the image and correctly display it on the screen.
            rotate_image_to_screen(&buf, top_screen.raw_framebuffer().ptr, WIDTH, HEIGHT);

            // We will only flush and swap the "camera" screen, since the other screen is handled by the `Console`.
            top_screen.flush_buffers();
            top_screen.swap_buffers();

            gfx.wait_for_vblank();
        }
    }
}

// The 3DS' screens are 2 vertical LCD panels rotated by 90 degrees.
// As such, we'll need to write a "vertical" image to the framebuffer to have it displayed properly.
// This functions rotates an horizontal image by 90 degrees to the right.
// This function is only supposed to be used in this example. In a real world application, the program should use the GPU to draw to the screen.
fn rotate_image_to_screen(src: &[u8], framebuf: *mut u8, width: usize, height: usize) {
    for j in 0..height {
        for i in 0..width {
            // Y-coordinate of where to draw in the frame buffer
            // Height must be esclusive of the upper end (otherwise, we'd be writing to the pixel one column to the right when having j=0)
            let draw_y = (height - 1) - j;
            // X-coordinate of where to draw in the frame buffer
            let draw_x = i;

            // Index of the pixel to draw within the image buffer
            let read_index = (j * width + i) * 2;

            // Initial index of where to draw in the frame buffer based on y and x coordinates
            let draw_index = (draw_x * height + draw_y) * 2; // This 2 stands for the number of bytes per pixel (16 bits)

            unsafe {
                // We'll work with pointers since the framebuffer is a raw pointer regardless.
                // The offsets are completely safe as long as the width and height are correct.
                let pixel_pointer = framebuf.offset(draw_index as isize);
                pixel_pointer.copy_from(src.as_ptr().offset(read_index as isize), 2);
            }
        }
    }
}
