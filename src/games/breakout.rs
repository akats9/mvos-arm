use core::error::Error;

use crate::{GPU_DEVICE, SCREENHEIGHT, SCREENWIDTH, THEME, dbg, drivers::uart::{self, echo_disable, echo_enable}, shell::start_shell, thread}; 

trait GameObject {
    fn draw(&self);
    fn clear(&self);
}

#[derive(Copy, Clone)]
struct Brick {
    position: (u32,u32),
    broken: bool,
    color: u32,
}

impl Brick {
    fn empty() -> Self {
        Self { position: (0,0), broken: false, color: 0}
    }
}

impl GameObject for Brick {
    fn draw(&self) {
        unsafe {
            let minx = self.position.0;
            let maxx = minx + 190;
            let miny = self.position.1;
            let maxy = miny + 30;
            let r = ((self.color >> 16) & 0xff) as u8;
            let g = ((self.color >> 8) & 0xff) as u8;
            let b = ((self.color >> 0) & 0xff) as u8;
            (*GPU_DEVICE.unwrap()).draw_rect(minx, maxx, miny, maxy, r, g, b);
        }
    }
    fn clear(&self) {
        unsafe {
            let minx = self.position.0;
            let maxx = minx + 190;
            let miny = self.position.1;
            let maxy = miny + 30;
            (*GPU_DEVICE.unwrap()).draw_rect(minx, maxx, miny, maxy, 0, 0, 0);
        }
    }
}

struct Paddle {
    position: (u32, u32),
    velocity: i32,
    color: u32, 
}

impl GameObject for Paddle {
    fn draw(&self) {
        unsafe {
            let r = ((self.color >> 16) & 0xff) as u8;
            let g = ((self.color >> 8) & 0xff) as u8;
            let b = ((self.color >> 0) & 0xff) as u8;
            (*GPU_DEVICE.unwrap()).draw_rect(self.position.0, self.position.0 + 150, self.position.1, self.position.1 + 20, r, g, b);
        }
        
    }

    fn clear(&self) {
        unsafe { (*GPU_DEVICE.unwrap()).draw_rect(self.position.0, self.position.0 + 150, self.position.1, self.position.1 + 20, 0, 0, 0); } 
    }
}

struct Ball {
    position: (u32, u32),
    velocity: (i32, i32),
    color: u32,
}

impl GameObject for Ball {
    fn draw(&self) {
        unsafe {
            let r = ((self.color >> 16) & 0xff) as u8;
            let g = ((self.color >> 8) & 0xff) as u8;
            let b = ((self.color >> 0) & 0xff) as u8;
            (*GPU_DEVICE.unwrap()).draw_rect(self.position.0, self.position.0 + 10, self.position.1, self.position.1 + 10, r, g, b);
        }
    }
    fn clear(&self) {
        unsafe { (*GPU_DEVICE.unwrap()).draw_rect(self.position.0, self.position.0 + 10, self.position.1, self.position.1 + 10, 0, 0, 0); }
    }
}

fn game_over() {
    unsafe {
        (*GPU_DEVICE.unwrap()).clear(0);
        (*GPU_DEVICE.unwrap()).as_text_mut().unwrap().draw_textbox("YOU WIN", 540, 300, 4, 0xffffff);
        thread::sleep(5000);
        start_shell();
    }
}

pub unsafe fn breakout_main() {
    // Init game scene
    (*GPU_DEVICE.unwrap()).clear(0);
    let theme = THEME;
    echo_disable();

    // Render bricks
    let mut bricks: [[Brick; 4] ; 6] = [[Brick::empty() ; 4] ; 6];
    for x in 0..6 {
        for y in 0..4 {
            bricks[x][y].color = theme.yellow();
            bricks[x][y].position = (20 + (x as u32)*210, 20 + (y as u32)*40);
            bricks[x][y].draw();
        }
    }

    // Render paddle
    let mut paddle = Paddle {position: (600, 600), velocity: 0, color: theme.white()};
    paddle.draw();

    // Render ball
    let mut ball = Ball {
        position: (SCREENWIDTH/2 as u32, SCREENHEIGHT/2 as u32),
        velocity: (1,2),
        color: theme.success(),
    };
    ball.draw();

    // Time vars
    let mut timestamp = 1; // Relative timestamp (1-1000 ms)
    let mut now: u32 = 0; // Absolute time in ms
    let mut lpbc: u32 = 0; // Last paddle-ball colission
    let mut lbyc: u32 = 0; // Last ball-ceiling or ball-floor collision
    let mut lbxc: u32 = 0; // Last ball-wall collision
    let mut lbbc: u32 = 0; // Last ball-brick collision

    // Score
    let mut score = 0;

    // Game loop
    loop {
        // Check for ball-wall collisions
        if (ball.position.1 == 0 || ball.position.1 >= SCREENHEIGHT - 10) && now - lbyc > 10 {
            dbg!("COLLISION");
            ball.velocity.1 = -ball.velocity.1;
            lbyc = now;
        } else if (ball.position.0 <= 10 || ball.position.0 >= SCREENWIDTH - 10) && now - lbxc > 100 {
            dbg!("WALL");
            ball.velocity.0 = -ball.velocity.0;
            lbxc = now;
        }

        // Update ball position
        if timestamp % 10 == 0 {
            ball.clear();
            ball.position.0 = ((ball.position.0 as i32 + ball.velocity.0) as u32).clamp(0, SCREENWIDTH);
            ball.position.1 = ((ball.position.1 as i32 + ball.velocity.1) as u32).clamp(0, SCREENHEIGHT);
            ball.draw();
        }

        // Check for keystroke and update paddle position
        let keystroke = uart::get_key_latest_gaming();
        if keystroke == 'd' && paddle.position.0 < SCREENWIDTH - 90 {
            paddle.clear();
            paddle.position.0 = (paddle.position.0 + 15).clamp(0, SCREENWIDTH - 150);
            paddle.velocity = 1;
            paddle.draw();
        }
        if keystroke == 'a' && paddle.position.0 > 15 {
            paddle.clear();
            paddle.position.0 = (paddle.position.0 - 15).clamp(0, SCREENWIDTH - 90);
            paddle.velocity = -1;
            paddle.draw();
        }

        // Check for paddle-ball collision
        if paddle.position.0 <= ball.position.0 && ball.position.0 <= paddle.position.0 + 150 && paddle.position.1 <= ball.position.1 && ball.position.1 <= paddle.position.1 + 20 && now - lpbc > 10 {
            ball.velocity.0 += paddle.velocity;
            ball.velocity.1 = -ball.velocity.1;
            lpbc = now;
            paddle.draw();
        }
        if timestamp % 1000 == 0 { paddle.velocity = 0 };

        // Check for ball-brick collision
        for x in 0..6 {
            for y in 0..4 {
                let pos = bricks[x][y].position;
                if pos.0 <= ball.position.0 && ball.position.0 <= pos.0 + 190 
                && pos.1 <= ball.position.1 && ball.position.1 <= pos.1 + 30
                && now - lbbc > 20 
                && bricks[x][y].broken == false {
                    dbg!("BRICK");
                    bricks[x][y].broken = true;
                    bricks[x][y].clear();
                    ball.velocity.1 = -ball.velocity.1;
                    lbbc = now;
                    score += 1;
                }
            }
        }

        // Reset ball
        if keystroke == 'r' {
            ball.clear();
            ball.position = (SCREENWIDTH/2 as u32, SCREENHEIGHT/2 as u32);
            ball.velocity = (0,2);
        }

        // Reset paddle 
        if keystroke == 'p' {
            paddle.clear();
            paddle.position = (600,600);
        }

        // Check score
        if score > 23 {
            game_over();
        }

        paddle.draw();

        // Tick clock
        thread::sleep(1);
        timestamp = (timestamp + 1) % 1000;
        now += 1;
    }
}