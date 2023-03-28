use crate::{HEIGHT, WIDTH};

pub struct Renderer {
    world_map: Vec<u32>,
    map_size: (usize, usize),
    position: (f64, f64),
    direction: (f64, f64),
    camera_plane: (f64, f64),
}

impl Renderer {
    pub fn new(world_map: Vec<u32>, map_x: usize, map_y: usize) -> Self {
        Self {
            world_map,
            map_size: (map_x, map_y),
            position: (2.0, 2.0),
            direction: (-1.0, 0.0),
            camera_plane: (0.0, 0.66),
        }
    }

    pub fn render(&mut self, frame: &mut [u8]) {
        let mut lines = Vec::new();
        for x in 0..WIDTH {
            let cam_x = (2 * x) as f64 / WIDTH as f64 - 1.0;
            let ray_dir = (
                self.direction.0 + self.camera_plane.0 * cam_x,
                self.direction.1 + self.camera_plane.1 * cam_x,
            );

            let mut map_position = (self.position.0 as i64, self.position.1 as i64);

            let delta_dist = ((1.0 / ray_dir.0).abs(), (1.0 / ray_dir.1).abs());

            let mut step = (0i64, 0i64);
            let mut side_dist = (0.0, 0.0);
            if ray_dir.0 < 0.0 {
                step.0 = -1;
                side_dist.0 = (self.position.0 - map_position.0 as f64) * delta_dist.0;
            } else {
                step.0 = 1;
                side_dist.0 = ((map_position.0 as f64 + 1.0) - self.position.0) * delta_dist.0;
            }
            if ray_dir.1 < 0.0 {
                step.1 = -1;
                side_dist.1 = (self.position.1 - map_position.1 as f64) * delta_dist.1;
            } else {
                step.1 = 1;
                side_dist.1 = ((map_position.1 as f64 + 1.0) - self.position.1) * delta_dist.1;
            }

            // dda
            let mut hit = false;
            let mut side = 0; // TODO: enum?
            while !hit {
                if side_dist.0 < side_dist.1 {
                    side_dist.0 += delta_dist.0;
                    map_position.0 += step.0;
                    side = 0;
                } else {
                    side_dist.1 += delta_dist.1;
                    map_position.1 += step.1;
                    side = 1;
                }
                if self.world_map
                    [map_position.0 as usize + map_position.1 as usize * self.map_size.1]
                    > 0
                {
                    hit = true;
                }
            }

            // calculate distance projected on camera direction
            let perp_wall_distance = if side == 0 {
                side_dist.0 - delta_dist.0
            } else {
                side_dist.1 - delta_dist.1
            };

            let line_height = (HEIGHT as f64 / perp_wall_distance) as usize;
            let draw_start = (-(line_height as isize) / 2 + HEIGHT as isize / 2).max(0);
            let draw_end = (line_height / 2 + HEIGHT / 2).min(HEIGHT - 1);

            let map_position = map_position.clamp(
                (0, 0),
                (self.map_size.0 as i64 - 1, self.map_size.1 as i64 - 1),
            );
            let color =
                self.world_map[map_position.0 as usize + map_position.1 as usize * self.map_size.1];
            let color_buf = [
                (color >> 24) as u8 / (side + 1),
                (color >> 16) as u8 / (side + 1),
                (color >> 8) as u8 / (side + 1),
                color as u8,
            ];
            lines.push(Line {
                color: color_buf,
                start: draw_start as usize,
                end: draw_end as usize,
            })
        }

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % WIDTH as usize;
            let y = i / WIDTH as usize;

            let line = lines[x];
            let rgba = if y > line.start && y < line.end {
                line.color
            } else {
                [0x30, 0x2c, 0x2e, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn set_position(&mut self, position: (f64, f64)) {
        let position = (
            position.0.clamp(0.0, self.map_size.0 as f64),
            position.1.clamp(0.0, self.map_size.1 as f64),
        );
        self.position = position;
    }

    pub fn add_position(&mut self, speed: f64) {
        let position = (
            self.position.0 + speed as f64 * self.direction.0,
            self.position.1 + speed as f64 * self.direction.1,
        );
        self.set_position(position);
    }

    pub fn add_rotation(&mut self, rotate: f64) {
        let old_dir = self.direction;
        self.direction.0 = self.direction.0 * rotate.cos() - self.direction.1 * rotate.sin();
        self.direction.1 = old_dir.0 * rotate.sin() + self.direction.1 * rotate.cos();
        let old_plane = self.camera_plane;
        self.camera_plane.0 =
            self.camera_plane.0 * rotate.cos() - self.camera_plane.1 * rotate.sin();
        self.camera_plane.1 = old_plane.0 * rotate.sin() + self.camera_plane.1 * rotate.cos();
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    color: [u8; 4],
    start: usize,
    end: usize,
}
