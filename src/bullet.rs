use ggez::graphics::{Canvas, DrawParam, Image, Rect};
use ggez::{Context, GameResult};

pub struct Bullet {
    pub sprite: Image,
    pub pos: [f32; 2],
    pub hitbox: Rect,
}

impl Bullet {
    pub fn new(ctx: &mut Context, spawn_pos: [f32; 2]) -> GameResult<Self> {
        let sprite = Image::from_path(ctx, "/bullet.png")?;
        let pos = [spawn_pos[0], spawn_pos[1]];
        let hitbox = Rect::new(
            spawn_pos[0],
            spawn_pos[1],
            sprite.width() as f32,
            sprite.height() as f32,
        );
        Ok(Self {
            sprite,
            pos,
            hitbox,
        })
    }

    pub fn update_hitbox(&mut self) {
        self.hitbox.x = self.pos[0];
        self.hitbox.y = self.pos[1];
    }

    pub fn draw(&self, canvas: &mut Canvas, _ctx: &mut Context) -> () {
        canvas.draw(&self.sprite, DrawParam::default().dest(self.pos));
    }

    pub fn on_shot(&mut self) {
        self.pos[1] -= 5.0;
        self.update_hitbox();
    }
}
