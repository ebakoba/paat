mod components;
mod localization;
mod messages;
mod model;
mod ascii;

use anyhow::Result;
use localization::setup_localization;
use log::error;
use model::Model;
use tuirealm::{PollStrategy, Update};

fn main() -> Result<()> {
    setup_localization()?;
    let mut model = Model::default();
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();
    while !model.quit {
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                error!("Failed due to: {}", err);
                model.quit = true;
            }
            Ok(messages) if messages.len() > 0 => {
                model.redraw = true;
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
    Ok(())
}
