use std::ffi;

use sokol::{app as sapp, audio as saudio, log as slog};

const NUM_SAMPLES: usize = 32;

// -------------------------------------------------------------------------------------------------

struct State {
    pub even_odd: u32,
    pub sample_pos: usize,
    pub samples: [f32; NUM_SAMPLES],
}

impl Default for State {
    fn default() -> Self {
        Self {
            even_odd: 0,
            sample_pos: 0,
            samples: [0.0; NUM_SAMPLES],
        }
    }
}

extern "C" fn init(_user_data: *mut ffi::c_void) {
    saudio::setup(&saudio::Desc {
        logger: saudio::Logger {
            func: Some(slog::slog_func),
            ..Default::default()
        },
        ..Default::default()
    });
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    let num_frames = saudio::expect();

    for _ in 0..num_frames {
        state.even_odd += 1;
        state.sample_pos += 1;

        if state.sample_pos == NUM_SAMPLES {
            state.sample_pos = 0;
            let _ = saudio::push(&(state.samples[0]), NUM_SAMPLES as _);
        }

        state.samples[state.sample_pos] = if 0 != (state.even_odd & 0x20) {
            0.1
        } else {
            -0.1
        };
    }
}

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    saudio::shutdown();

    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

// -------------------------------------------------------------------------------------------------

pub fn main() {
    let state = Box::new(State::default());
    let user_data = Box::into_raw(state) as *mut ffi::c_void;

    sapp::run(&sapp::Desc {
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        cleanup_userdata_cb: Some(cleanup),
        user_data,
        width: 640,
        height: 480,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        window_title: c"audio test".as_ptr(),
        logger: sapp::Logger {
            func: Some(slog::slog_func),
            ..Default::default()
        },
        ..Default::default()
    });
}
