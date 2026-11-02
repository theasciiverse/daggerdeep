//! Session logging, an Asciiverse addition: `RUST_LOG=debug daggerdeep
//! 2>daggerdeep.log` records every input event as it arrives, state and
//! level changes, and a snapshot of the game after each turn (at most
//! twice a second in a hosted game, whose loop runs every 50 ms).

use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event};

use crate::game::{Game, GameState};

/// `event::read`, logging every key and mouse event before it is handled.
pub fn read() -> io::Result<Event> {
    let ev = event::read()?;
    match &ev {
        Event::Key(k) => log::debug!("key {:?} {:?} {:?}", k.code, k.kind, k.modifiers),
        Event::Mouse(m) => log::debug!("mouse {:?} at {},{}", m.kind, m.column, m.row),
        Event::Resize(w, h) => log::info!("resize {w}x{h}"),
        _ => {}
    }
    Ok(ev)
}

fn state_name(s: GameState) -> &'static str {
    match s {
        GameState::Playing => "playing",
        GameState::GameOver => "game over",
        GameState::Victory => "victory",
        GameState::ConfirmQuit => "confirm quit",
        GameState::Disconnected => "disconnected",
    }
}

/// Watches a game between turns.
pub struct Watch {
    state: Option<GameState>,
    level: u8,
    last: Instant,
}

impl Watch {
    pub fn new() -> Watch {
        Watch { state: None, level: 0, last: Instant::now() - Duration::from_secs(1) }
    }

    pub fn observe(&mut self, g: &Game) {
        if self.state != Some(g.state) {
            log::info!("state {}", state_name(g.state));
            self.state = Some(g.state);
        }
        if self.level != g.level {
            log::info!("level {}", g.level);
            self.level = g.level;
        }
        if self.last.elapsed() < Duration::from_millis(500) && g.multiplayer {
            return;
        }
        self.last = Instant::now();
        let alive: Vec<_> = g.monsters.iter().filter(|m| m.alive).map(|m| (m.kind.char(), m.x, m.y, m.hp)).collect();
        let p2 = g.player2.as_ref().map(|p| (p.x, p.y, p.hp, p.max_hp, p.respawns_left));
        log::debug!(
            "snapshot level {} state {} p1 ({},{}) hp {}/{} respawns {} p2 {:?} monsters {:?} status {:?}",
            g.level,
            state_name(g.state),
            g.player.x,
            g.player.y,
            g.player.hp,
            g.player.max_hp,
            g.player.respawns_left,
            p2,
            alive,
            g.status_msg
        );
    }
}
