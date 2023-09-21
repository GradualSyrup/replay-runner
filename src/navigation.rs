use std::ffi::CStr;
use smash::app::Fighter_is_ready_go;
use crate::playaid;

// Track where we are in the navigation sequence
#[derive(PartialEq)]
pub enum CurrentNavigation {
    // Starts with Main Menu Navigation
    MainWaitingForLoad, // Loading
    MainOnMelee, // Pressing Down
    MainOnSpirits, // Pressing Right
    MainOnOnline, // Pressing A
    MainWaitingOnline, // Waiting
    MainInOnline, // Pressing Down
    MainOnSharedContent, // Pressing A
    // Moves to Shared Content Navigation
    ScWaitingForLoad, // Pressing X (for 10 seconds for load times)
    ScSearchSubmenuTop, // Press Up Here (press once)
    ScSearchSubmenuBottom, // Pressing A (until keyboard shows up)
    ScKeyboard, // Should be automated - unused
    ScSearchResults, // See if "No content found." is an MBST, use to determine if bad ID 
        // - mnu_share_search_result_title -> HoverReplay
    _ScBadId, // Press B Here (Once, after waiting 3 seconds), then wait 3 seconds, then we're at ScSearchSubmenuBottom - unused
    ScHoverReplay, // Pressing A, until Loading MBST or game start
        // mnu_share_detail_stage, pop_online_communicating_mini once clicked again and we're loading - use that to go to waiting for game
    ScWaitingForGame, // Holding X, until GO
    ScGO, // Press X+Down to hide overlay,
    ScPlayback, // Pressing B, until we scene transition back to previous scene
    // Move 
    DoneHoverPlay, // Pressing B until we're back at the Search Menu, then we're at ScSearchSubmenuBottom - unused
}

pub static mut NAV: CurrentNavigation = CurrentNavigation::MainWaitingForLoad; //CurrentNavigation::MainWaitingForLoad; //CurrentNavigation::Debug;

// Hook MBST get label to understand where we are in menu navigation currently
#[skyline::hook(offset = 0x3778af0)]
unsafe fn mbst_get_label(layout_view: *mut u64, label_string: *mut u8) {
    let label_rust_str = CStr::from_ptr(label_string).to_str().unwrap();
    if label_rust_str == "mnu_top_help_melee" && NAV == CurrentNavigation::MainWaitingForLoad {
        println!("Melee selected - should move down!");
        NAV = CurrentNavigation::MainOnMelee;
    }
    if label_rust_str == "mnu_top_help_spirits" && NAV == CurrentNavigation::MainOnMelee {
        println!("Spirits selected - should move right!");
        NAV = CurrentNavigation::MainOnSpirits;
    }
    if label_rust_str == "mnu_top_help_online" && NAV == CurrentNavigation::MainOnSpirits {
        println!("Online selected - should enter!");
        NAV = CurrentNavigation::MainOnOnline;
    }
    if label_rust_str == "pop_online_connecting" && NAV == CurrentNavigation::MainOnOnline {
        println!("In a cancel pop-up - don't press anything!");
        NAV = CurrentNavigation::MainWaitingOnline;
    }
    if label_rust_str == "mnu_onl_top_help_onl_melee" && (NAV == CurrentNavigation::MainOnOnline || NAV == CurrentNavigation::MainWaitingOnline) {
        println!("Online entered - should move down!");
        NAV = CurrentNavigation::MainInOnline;
    }
    if label_rust_str == "mnu_onl_top_help_contribution" && NAV == CurrentNavigation::MainInOnline {
        println!("SC selected - should enter!");
        NAV = CurrentNavigation::MainOnSharedContent;
    }
    if label_rust_str == "mnu_share_cat_movie" && NAV == CurrentNavigation::ScWaitingForLoad {
        println!("In SubMenu!");
        NAV = CurrentNavigation::ScSearchSubmenuTop;
    }
    if label_rust_str == "mnu_share_search_result_title" && NAV == CurrentNavigation::ScSearchResults {
        println!("In Search Results!");
        NAV = CurrentNavigation::ScHoverReplay;
    }
    if label_rust_str == "pop_share_no_post" && NAV == CurrentNavigation::ScHoverReplay {
        println!("Bad ID!");
        playaid::handle_bad_id();
        NAV = CurrentNavigation::ScPlayback;
    }
    if label_rust_str == "pop_online_communicating_mini" && NAV == CurrentNavigation::ScHoverReplay {
        println!("Game Starting!");
        NAV = CurrentNavigation::ScWaitingForGame;
    }
    if label_rust_str == "mel_pause_filter_00" && NAV == CurrentNavigation::ScWaitingForGame {
        if Fighter_is_ready_go() {
            println!("GO!");
            NAV = CurrentNavigation::ScGO;
        }
    }
    if label_rust_str == "mnu_share_cat_movie" && (NAV == CurrentNavigation::DoneHoverPlay || NAV == CurrentNavigation::ScPlayback) {
        println!("Returned to SubMenu!");
        playaid::replay_done();
        NAV = CurrentNavigation::ScSearchSubmenuBottom;
    }
    call_original!(layout_view, label_string);
}

static mut WAIT_COUNT: u64 = 0;
const WAIT_MAX: u64 = 40;

// See if we need to keep delaying inputs
// This likely gets incremented more than once a frame since we're using the npad checking function, so give it a large delay
pub unsafe fn should_wait() -> bool {
    println!("Wait Count = {}", WAIT_COUNT);
    if WAIT_COUNT < WAIT_MAX {
        WAIT_COUNT += 1;
        return true;
    }
    WAIT_COUNT = 0;
    false
}

pub fn init() {
    skyline::install_hooks!(
        mbst_get_label
    );
}