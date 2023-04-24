use std::ops::Range;
use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::terminal::Window;
use space_invaders::army::Army;
use space_invaders::drawable::Drawable;
use space_invaders::player::Player;

fn main() {
    let mut fps_counter = FPSCounter::default();
    let mut app = App::default();

    let win_size = Vec2::xy(40, 20);

    let mut player = Player::new(get_player_position(win_size));
    let mut army = Army::new(get_aliens_range(win_size));

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) | KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::Right => player.move_right(),
                Key::Left => player.move_left(),
                Key::Space => player.shoot(),
                _ => (),
            }
        }

        fps_counter.update();

        let mut pencil = Pencil::new(window.canvas_mut());

        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1));
        pencil.draw_rect(&RectCharset::simple_round_lines(), Vec2::zero(), win_size);

        player.draw(&mut pencil);
        player.draw_shoots(&mut pencil, &mut army.aliens);
        army.draw_aliens(&mut pencil);

        if army.dead() {
            app_state.stop();
        }
    });
}

fn get_player_position(win_size: Vec2) -> Vec2 {
    // Center the player
    Vec2::xy(win_size.x / 2, win_size.y - 2)
}

fn get_aliens_range(win_size: Vec2) -> Range<Vec2> {
    Vec2::xy(5, 2)..Vec2::xy(35, 10)
}
