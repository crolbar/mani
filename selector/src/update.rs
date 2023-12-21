use crossterm::event::{KeyCode, Event, self, MouseEventKind};
use std::io::Result;

use crate::{app::Selector, tui::Tui};

pub fn update(app: &mut Selector, _tui: &mut Tui) -> Result<()> {
    if let Ok(event) = event::read() {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Down => app.sel_next_item(),
                KeyCode::Up => app.sel_prev_item(),
                KeyCode::Esc | KeyCode::Enter => app.exit = true,
                //KeyCode::Char(char) => app
                _ => ()
            }
        } else 

        if let Event::Mouse(mouse_ev) = event {
            match mouse_ev.kind {
                MouseEventKind::ScrollDown => app.sel_next_item(),
                MouseEventKind::ScrollUp => app.sel_prev_item(),
                MouseEventKind::Down(_) => app.handle_mb_down(mouse_ev),
                _ => ()
            }
        }
    }
    Ok(())
}
