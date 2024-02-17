use ansi_term::{Style, Colour::Fixed};
use zellij_tile::prelude::*;
use zellij_tile::prelude::actions::Action;
use zellij_tile::prelude::InputMode;
use zellij_tile::prelude::Direction;
use zellij_tile::prelude::Resize;

use std::collections::{HashMap, BTreeMap};

#[derive(Default)]
struct State {
    mode_log: HashMap<String, usize>,
    tabs: Vec<String>,
    mode_info: ModeInfo,
    test_runs: usize,
    userspace_configuration: BTreeMap<String, String>,
}

register_plugin!(State);

fn mode_name(mode: InputMode) -> String {
    (match mode {
        InputMode::Normal => "Normal",
        InputMode::Locked => "Locked",
        InputMode::Resize => "Resize",
        InputMode::Pane => "Pane",
        InputMode::Tab => "Tab",
        InputMode::Scroll => "Scroll",
        InputMode::EnterSearch => "Enter search",
        InputMode::Search => "Search",
        InputMode::RenameTab => "Rename tab",
        InputMode::RenamePane => "Rename pane",
        InputMode::Session => "Session",
        InputMode::Move => "Move",
        InputMode::Prompt => "Prompt",
        InputMode::Tmux => "Tmux"
    }).to_string()
}

fn direction_name(direction: Direction) -> String {
    (match direction {
        Direction::Up => "up",
        Direction::Left => "left",
        Direction::Right => "right",
        Direction::Down => "down"
    }).to_string()
}

fn resize_name(resize:Resize) -> String {
    (match resize {
        Resize::Increase => "increase",
        Resize::Decrease => "decrease"
    }).to_string()
}

fn action_name(action: Action) -> String {
    match action {
        Action::Quit => "Quit".to_string(),
        Action::SwitchToMode(m) => format!("{} mode", mode_name(m)),
        Action::Resize(res, dir) => format!("{} {}", 
                                            resize_name(res), 
                                            dir.map(direction_name).unwrap_or("".to_string())),
        Action::FocusNextPane => "Next pane".to_string(),
        Action::FocusPreviousPane => "Previous pane".to_string(),
        Action::SwitchFocus => "Switch focus".to_string(),
        Action::MoveFocus(dir) => format!("Move focus {}", direction_name(dir)), 
        _ => "unknown".to_string()
    }
}

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.userspace_configuration = configuration;
        // we need the ReadApplicationState permission to receive the ModeUpdate and TabUpdate
        // events
        // we need the RunCommands permission to run "cargo test" in a floating window
        request_permission(&[PermissionType::ReadApplicationState, PermissionType::RunCommands]);
        subscribe(&[EventType::ModeUpdate, EventType::TabUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    should_render = true;
                }
                self.mode_info = mode_info;
            }
            Event::TabUpdate(tab_info) => {
                self.tabs = tab_info.iter().map(|t| t.name.clone()).collect();
                should_render = true;
            }
            _ => (),
        };
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        // let kb = self.mode_info.get_mode_keybinds();
        let kb = self.mode_info.get_keybinds_for_mode(self.mode_info.mode);


        fn is_mode_change(action: Action) -> bool {
            match action {
                Action::SwitchToMode(_m) => true,
                _ => false
            }
        }

        // Get only the mode change key bindings
        let mode_change_kbs = kb.iter().filter(|(_key, acvec)| { is_mode_change(acvec[0].clone()) });

        // Get all other key bindings
        let regular_kbs = kb.iter().filter(|(_key, acvec)| { !is_mode_change(acvec[0].clone()) });

        // Print mode changes first
        for (key, acvec) in mode_change_kbs {
            println!("{}: {}", color_bold(ORANGE, &key.to_string()), action_name(acvec[0].clone())); 
        }

        for (key, acvec) in regular_kbs {
            println!("{}: {}", color_bold(CYAN, &key.to_string()), action_name(acvec[0].clone())); 
        }
    }
}

pub const CYAN: u8 = 51;
pub const GRAY_LIGHT: u8 = 238;
pub const GRAY_DARK: u8 = 245;
pub const WHITE: u8 = 15;
pub const BLACK: u8 = 16;
pub const RED: u8 = 124;
pub const GREEN: u8 = 154;
pub const ORANGE: u8 = 166;

fn color_bold(color: u8, text: &str) -> String {
    format!("{}", Style::new().fg(Fixed(color)).bold().paint(text))
}
