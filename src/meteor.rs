use ggez::graphics::{Canvas, DrawParam, Image, Rect};
use ggez::{Context, GameResult};

use rand::Rng;
use rand::rngs::ThreadRng;

pub struct Meteor {
    pub sprite: Image,
    pub pos: [f32; 2],
    pub hitbox: Rect,
}

impl Meteor {
    pub fn new(ctx: &mut Context, screen_size: [f32; 2]) -> GameResult<Self> {
        let mut rng: ThreadRng = rand::thread_rng();

        let rand_x: f32 = rng.gen_range(0.0..=screen_size[0] - 200.0);
        let rand_y: f32 = rng.gen_range(-500.0..=0.0);

        let sprite = Image::from_path(ctx, "/meteor.png")?;
        let pos = [rand_x, rand_y];
        let hitbox = Rect::new(
            rand_x,
            rand_y,
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

    pub fn move_meteor(&mut self) {
        self.pos[1] += 2.0;
        self.update_hitbox();
    }
}
