use ruscii::drawing::Pencil;
use ruscii::spatial::Vec2;

pub trait Drawable {
    fn get_character(&self) -> &str;

    fn get_position(&self) -> Vec2;

    fn draw(&self, pencil: &mut Pencil) {
        pencil.draw_text(self.get_character(), self.get_position());
    }
}
