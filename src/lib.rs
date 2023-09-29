use std::ops::Add;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[derive(Clone, Debug)]
pub struct Line {
    pub start: Vector,
    pub end: Vector,
}

#[derive(Clone, Debug)]
pub struct Vector {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
pub struct Vector2f {
    x: f32,
    y: f32,
}

pub struct Scene {
    pub points: Vec<Vector>,
}

fn binomial_coefficient(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }

    let mut res = 1;
    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }

    res
}

impl Add for Vector2f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Color {
    fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Line {
    pub fn new(start: Vector, end: Vector) -> Self {
        Line { start, end }
    }
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector { x, y }
    }
}

impl Scene {
    pub fn new() -> Self {
        Scene { points: Vec::new() }
    }

    fn draw_cell(&self, screen: &mut [u8], position: &Vector, color: &Color) {
        let index = 4 * (position.y * WIDTH as i32 + position.x) as usize;
        screen[index] = color.red;
        screen[index + 1] = color.green;
        screen[index + 2] = color.blue;
        screen[index + 3] = color.alpha;
    }

    fn draw_line(&self, screen: &mut [u8], line: &Line, color: Color) {
        let mut current = Vector::new(line.start.x, line.start.y);
        let delta = Vector::new(
            (line.end.x - line.start.x).abs(),
            (line.end.y - line.start.y).abs(),
        );
        let step = Vector::new(
            if line.start.x < line.end.x { 1 } else { -1 },
            if line.start.y < line.end.y { 1 } else { -1 },
        );
        let mut err = delta.x - delta.y;

        loop {
            self.draw_cell(screen, &current, &color);

            if current.x == line.end.x && current.y == line.end.y {
                break;
            }

            let e2 = 2 * err;

            if e2 > -delta.y {
                err -= delta.y;
                current.x += step.x;
            }

            if e2 < delta.x {
                err += delta.x;
                current.y += step.y;
            }
        }
    }

    fn draw_grid(&self, screen: &mut [u8]) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.draw_cell(
                    screen,
                    &Vector::new(x as i32, y as i32),
                    &Color::new(71, 110, 252, 255),
                )
            }
        }
        for y in 0..WIDTH / 16 {
            self.draw_line(
                screen,
                &Line::new(
                    Vector::new(y as i32 * 16, 0),
                    Vector::new(y as i32 * 16, HEIGHT as i32 - 1),
                ),
                Color::new(100, 132, 250, 255),
            );
        }
        for x in 0..HEIGHT / 16 + 1 {
            self.draw_line(
                screen,
                &Line::new(
                    Vector::new(0, x as i32 * 16),
                    Vector::new(WIDTH as i32, x as i32 * 16),
                ),
                Color::new(100, 132, 250, 255),
            );
        }
    }

    fn bezier_curve(&self, t: f32) -> Vector {
        let n = self.points.len() - 1;
        let mut sum = Vector2f { x: 0.0, y: 0.0 };

        for (i, point) in self.points.iter().enumerate() {
            let binomial = binomial_coefficient(n, i);
            let bernstein = binomial as f32 * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32);
            sum = sum
                + Vector2f {
                    x: point.x as f32 * bernstein,
                    y: point.y as f32 * bernstein,
                };
        }

        Vector::new(sum.x as i32, sum.y as i32)
    }

    pub fn print(&self, screen: &mut [u8]) {
        self.draw_grid(screen);

        if self.points.len() > 2 {
            let mut points: Vec<Vector> = Vec::new();

            for i in 0..=100 {
                let t = i as f32 / 100.0;
                points.push(self.bezier_curve(t));
            }

            for i in 1..points.len() {
                self.draw_line(
                    screen,
                    &Line::new(points[i - 1].clone(), points[i].clone()),
                    Color::new(255, 255, 255, 255),
                );
            }
        }

        if self.points.len() != 0 {
            for i in 1..self.points.len() {
                self.draw_line(
                    screen,
                    &Line::new(self.points[i - 1].clone(), self.points[i].clone()),
                    Color::new(255, 255, 255, 100),
                );
            }
        }
    }
}
