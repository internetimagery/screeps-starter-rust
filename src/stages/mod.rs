// Different game loops for different game stages
use log::*;

pub struct Stage {
    prev: Option<Box<Stage>>,
    logic: Box<dyn StageLoop>,
}

impl Stage {
    pub fn new(stage_loop: Box<dyn StageLoop>) -> Self {
        Self {
            prev: None,
            logic: stage_loop,
        }
    }
    // pub fn next(self, stage_loop: Box<dyn StageLoop>) -> Self {
    //     Self{prev:Some(Box::new(self)), logic:stage_loop}
    // }
    pub fn go(&self) -> bool {
        match &self.prev {
            None => self.execute(),
            Some(prev) => {
                if prev.go() {
                    true
                } else {
                    self.execute()
                }
            }
        }
    }
    fn execute(&self) -> bool {
        if self.logic.can_run() {
            self.logic.run();
            true // Stop further chain execution
        } else {
            false // Continue execution
        }
    }
}

pub trait StageLoop {
    fn can_run(&self) -> bool {
        false
    }
    fn run(&self);
}

// Initial base building
pub struct BuildBase {}

impl StageLoop for BuildBase {
    fn can_run(&self) -> bool {
        true
    }
    fn run(&self) {
        debug!("Running HI THERE");
    }
}
