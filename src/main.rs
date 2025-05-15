use std::ffi;

use sokol::{
    audio::{self as saudio},
    log as slog,
};

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct AudioState {
    pub even_odd: u32,
}

// -------------------------------------------------------------------------------------------------

extern "C" fn audio_callback(
    buffer: *mut f32,
    num_frames: i32,
    num_channels: i32,
    userdata: *mut core::ffi::c_void,
) {
    let state = unsafe { &mut *(userdata as *mut AudioState) };

    let buf_slice: &mut [f32] = unsafe {
        std::slice::from_raw_parts_mut(buffer, num_frames as usize * num_channels as usize)
    };

    for frame in buf_slice.chunks_exact_mut(num_channels as usize) {
        state.even_odd += 1;
        for sample in frame {
            *sample = if 0 != (state.even_odd & 0x20) {
                0.1
            } else {
                -0.1
            };
        }
    }
}

#[no_mangle]
pub extern "C" fn init_audio() {
    let state = Box::new(AudioState::default());
    let user_data = Box::into_raw(state) as *mut ffi::c_void;

    saudio::setup(&saudio::Desc {
        num_channels: 2,
        stream_userdata_cb: Some(audio_callback),
        user_data,
        logger: saudio::Logger {
            func: Some(slog::slog_func),
            ..Default::default()
        },
        ..Default::default()
    });
}

#[no_mangle]
pub extern "C" fn shutdown_audio() {
    saudio::shutdown();
    let _ = unsafe { Box::from_raw(saudio::userdata() as *mut AudioState) };
}

// -------------------------------------------------------------------------------------------------

fn main() {
    // unused
}
