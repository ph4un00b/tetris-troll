use crate::shared::{Evt, StateMachine};

/*
 * @refactor: this is a manual minimal state machine
 * if you have a potential improvement over here
 * please go ahead ╰(*°▽°*)╯
 */
impl StateMachine for GameMachine {
    fn send(&mut self, evt: &Evt) {
        match &self.state {
            Manager::Idle => {
                if matches!(evt, Evt::Menu) {
                    self.state = Manager::MainEntry;
                }
            }
            Manager::MainEntry => {
                if matches!(evt, Evt::Menu) {
                    self.state = Manager::Main;
                }
            }
            Manager::Main => {
                if matches!(evt, Evt::Exit) {
                    self.state = Manager::Exit
                }
                if matches!(evt, Evt::Play) {
                    self.state = Manager::PlayingEntry
                }
            }
            Manager::PlayingEntry => {
                if matches!(evt, Evt::Play) {
                    self.state = Manager::Playing;
                }
            }
            Manager::Playing => {
                if matches!(evt, Evt::DTap) {
                    self.state = Manager::PlayingExit(Evt::DTap);
                }
                if matches!(evt, Evt::Pause) {
                    self.state = Manager::PlayingExit(Evt::Pause);
                }
                if matches!(evt, Evt::Dead) {
                    self.state = Manager::PlayingExit(Evt::Dead);
                }
            }
            Manager::PlayingExit(_e) => {
                if matches!(evt, Evt::Pause) {
                    self.state = Manager::PausedEntry;
                }
                if matches!(evt, Evt::Dead) {
                    self.state = Manager::GameOver;
                }
            }
            Manager::PausedEntry => {
                if matches!(evt, Evt::Pause) {
                    self.state = Manager::Paused;
                }
            }
            Manager::Paused => {
                if matches!(evt, Evt::DTap) {
                    self.state = Manager::PlayingEntry;
                }
                if matches!(evt, Evt::Play) {
                    self.state = Manager::PlayingEntry;
                }
            }
            Manager::GameOver => {
                if matches!(evt, Evt::Menu) {
                    self.state = Manager::MainEntry;
                }
            }
            Manager::Exit => (),
        }
    }
}

pub struct GameMachine {
    pub state: Manager,
}

impl GameMachine {
    pub async fn new() -> Self {
        Self {
            state: Manager::Idle,
        }
    }
}
pub enum Manager {
    Idle,
    MainEntry,
    Main,
    PlayingEntry,
    Playing,
    PlayingExit(Evt),
    PausedEntry,
    Paused,
    GameOver,
    Exit,
}
