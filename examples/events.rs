/// Demostrats using hyprland-rs to listen for events
/// 
/// Usage: cargo run --example events

use hyprland::dispatch;
use hyprland::dispatch::{Dispatch, DispatchType, FullscreenType};
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;

fn main() -> hyprland::Result<()> {
        // Create a event listener
        let mut event_listener = EventListener::new();

        event_listener.add_active_window_change_handler(|data| {
            println!("{data:#?}");
        });
    
        event_listener.add_workspace_change_handler(|state| {
            println!("Workspace changed: {state:#?}");
        });

        event_listener.add_fullscreen_state_change_handler(|fstate| {
            println!("Window {} fullscreen", if fstate { "is" } else { "is not" });
        });

        event_listener.add_active_monitor_change_handler(|state| {
            println!("Monitor state: {state:#?}");
        });
    
        // add event, yes functions and closures both work!
        event_listener.add_workspace_change_handler(|id| println!("workspace changed to {id:#?}"));
    
        // and execute the function
        // here we are using the blocking variant
        // but there is a async version too
        event_listener.start_listener()?;

        return Ok(());
}