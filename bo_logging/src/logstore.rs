use super::*;
use bracket_lib::terminal::*;
use parking_lot::Mutex;

static LOG: Mutex<Vec<Vec<LogFragment>>> = Mutex::new(Vec::new());

pub fn clear_log() { LOG.lock().clear(); }
pub fn append_entry(fragments: Vec<LogFragment>) { LOG.lock().push(fragments); }

pub fn print_log(draw_batch: &mut DrawBatch, log_rect: Rect) {
    let mut y = log_rect.y1 + 1;
    let mut x = log_rect.x1 + 1;

    LOG.lock().iter().rev().take(5).for_each(|log| {
        log.iter().for_each(|frag| {
            draw_batch.print_color_with_z(
                Point::new(x, y),
                &frag.text,
                ColorPair::new(frag.color, BLACK),
                100_000,
            );
            x += frag.text.len() as i32 + 1;
        });

        y += 1;
        x = log_rect.x1 + 1;
    });
}

pub fn clone_log() -> Vec<Vec<LogFragment>> { LOG.lock().clone() }

pub fn restore_log(log: &mut Vec<Vec<LogFragment>>) {
    LOG.lock().clear();
    LOG.lock().append(log);
}
