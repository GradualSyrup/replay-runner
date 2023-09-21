#![feature(pointer_byte_offsets)]
#![feature(new_uninit)]
#![feature(vec_into_raw_parts)]
use crate::navigation::CurrentNavigation;

mod navigation;
mod input;
mod keyboard;
mod playaid;

#[repr(C)]
#[derive(Debug)]
pub struct FixedBaseString<const N: usize> {
    fnv: u32,
    string_len: u32,
    string: [u8; N],
}

#[repr(C)]
#[derive(Debug)]
pub struct SceneQueue {
    end: *const u64,
    start: *const u64,
    count: usize,
    active_scene: FixedBaseString<64>,
    previous_scene: FixedBaseString<64>
}

#[skyline::hook(offset = 0x3724c10)]
fn change_scene_sequence(queue: &SceneQueue, fnv1: &mut FixedBaseString<64>, fnv2: &mut FixedBaseString<64>, parameters: *const u8) {
    if &fnv1.string[0..24] == b"OnlineShareSequenceScene" && &fnv2.string[0..17] == b"MenuSequenceScene" {
        println!("Made it to Shared Content!");
        unsafe {
            navigation::NAV = CurrentNavigation::ScWaitingForLoad;
        }
    }
    call_original!(queue, fnv1, fnv2, parameters);
}

#[skyline::from_offset(0x39c4bb0)]
fn begin_auto_sleep_disabled();

#[skyline::hook(offset = 0x39c4bd0)]
fn end_auto_sleep_disabled() {
    // We don't want to auto-sleep ever, so don't let this end
}

#[skyline::hook(offset = 0x39c4bc0)]
fn kill_backlight() {
    // We don't want to kill backlight ever, so don't let this happen
}

fn hook_panic() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        let err_msg = format!("thread has panicked at '{}', {}", msg, location);
        skyline::error::show_error(
            69,
            "Skyline plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
}

#[skyline::main(name = "replay-runner")]
pub fn main() {
    // Add panic hook
    hook_panic();
    
    // Initialize hooks for navigation and keyboard
    navigation::init();
    keyboard::init();

    // Initialize hooks for scene usage
    skyline::install_hooks!(
        change_scene_sequence,
        kill_backlight,
        end_auto_sleep_disabled
    );

    // Initialize hooks for input (from result_screen_skip)
    std::thread::sleep(std::time::Duration::from_secs(20)); //makes it not crash on startup with arcrop bc ???
    println!("[Auto-Replay] Installing input hook...");
    unsafe {
        if (input::add_nn_hid_hook as *const ()).is_null() {
            panic!("The NN-HID hook plugin could not be found and is required to add NRO hooks. Make sure libnn_hid_hook.nro is installed.");
        }
        input::add_nn_hid_hook(input::handle_get_npad_state_start);

        println!("Disabling Auto Sleep");
        begin_auto_sleep_disabled()
    }
}
