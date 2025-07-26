use std::time::Duration;

use ggez::graphics::{self, Image, Rect, DrawParam, Canvas, Color};
use ggez::{Context, GameResult};

pub struct Player{
    pub sprite: Image,
    pub pos: [f32; 2],
    pub hitbox: Rect,

    pub health: f32,

    last_damaged: Duration,
}

impl Player{
    pub fn new(ctx: &mut Context) -> GameResult<Self>{
        let sprite = Image::from_path(ctx, "/player.png")?;
        let pos = [100.0, 100.0];
        let hitbox = Rect::new(pos[0], pos[1], sprite.width() as f32, sprite.height() as f32);
        let health: f32 = 100.0;
        Ok(Self { sprite, pos, hitbox, health, last_damaged: ctx.time.time_since_start() })
    }

    pub fn update_hitbox(&mut self) {
        self.hitbox.x = self.pos[0];
        self.hitbox.y = self.pos[1];
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) -> () {
        canvas.draw(&self.sprite, DrawParam::default().dest(self.pos));

        let hitbox_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            self.hitbox,
            Color::RED,
        ).unwrap();

        canvas.draw(&hitbox_mesh, DrawParam::default());
    }

    pub fn move_player(&mut self, direction: [f32; 2]) -> (){
        self.pos[0] += direction[0];
        self.pos[1] += direction[1];
        self.update_hitbox();
    }

    pub fn take_damage(&mut self, damage: f32, now: Duration) -> (){
        let elapsed: Duration = now - self.last_damaged;
        if elapsed < Duration::from_secs(1) {
            return;
        }
        self.health -= damage;
        self.health = self.health.clamp(0.0, 100.0);

        self.last_damaged = now;
        println!("Health: {}, Damage taken: {}", self.health,  damage);
    }
}

