use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyCode;
use std::time::Duration;

mod player;
use player::Player;

mod bullet;
use bullet::Bullet;

mod meteor;
use meteor::Meteor;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("space_shooter", "VortexDrags")
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx).expect("Failed to create game");
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    bullets: Vec<Bullet>,
    meteors: Vec<Meteor>,

    last_meteor_spawn_time: Duration,

    player: Player,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let player = Player::new(ctx)?;
        Ok(Self {
            bullets: vec![],
            meteors: vec![],
            last_meteor_spawn_time: ctx.time.time_since_start(),
            player,
        })
    }
}

impl EventHandler for MyGame {
   fn update(&mut self, ctx: &mut Context) -> GameResult {
        let (width, height) = ctx.gfx.drawable_size();

        if ctx.keyboard.is_key_pressed(KeyCode::W) {
            self.player.move_player([0.0, 1.0]);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::S) {
            self.player.move_player([0.0, -1.0]);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            self.player.move_player([-1.0, 0.0]);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            self.player.move_player([-1.0, 0.0]);
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Space){
            self.bullets.push(Bullet::new(ctx, self.player.pos)?);
        }

        for bullet in &mut self.bullets{
            bullet.on_shot();
        }

        let now: Duration = ctx.time.time_since_start();
        let elapsed: Duration = now - self.last_meteor_spawn_time;

        if elapsed >= Duration::from_secs(1) {
            self.meteors.push(Meteor::new(ctx, [width, height])?);
            self.last_meteor_spawn_time = now;
        }

        for meteor in &mut self.meteors{
            meteor.move_meteor();
        }

        let mut meteors_to_delete: Vec<usize> = vec![];
        for i in 0..self.bullets.len(){
            for j in 0..self.meteors.len(){
                if self.bullets[i].hitbox.overlaps(&self.meteors[j].hitbox){
                    meteors_to_delete.push(j);
                }
            }
        }
        meteors_to_delete.sort_by(|a, b| b.cmp(a));
        for meteors in &meteors_to_delete{
            self.meteors.remove(*meteors);
        }

        for i in 0..self.meteors.len(){
            if self.player.hitbox.overlaps(&self.meteors[i].hitbox){
                self.player.take_damage(10.0, now);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        self.player.draw(&mut canvas, ctx);

        for meteor in &mut self.meteors{
            meteor.draw(&mut canvas, ctx);
        }

        for bullet in &mut self.bullets{
            bullet.draw(&mut canvas, ctx);
        }

        canvas.finish(ctx)
    }
}