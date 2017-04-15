use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    pub fn length(&self) -> f32 {
        ((self.x).powi(2) + (self.y).powi(2)).sqrt()
    }

    pub fn scale(&self, s: f32) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }

    pub fn normalized(&self) -> Vec2 {
        self.scale(1.0 / self.length())
    }

    pub fn horizontal_sum(&self) -> f32 {
        self.x + self.y
    }

    pub fn abs(&self) -> Vec2 {
        Vec2::new(self.x.abs(), self.y.abs())
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)

    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}


pub struct Ticker {
    // ticks since last fire
    ticks: usize,

    // every period-th tick should fire
    period: usize,

    // Fire on first tick always
    first: bool
}

impl Ticker {
    pub fn new(period: usize) -> Ticker {
        Ticker{ ticks: 0, period: period, first: true }
    }

    pub fn tick(&mut self) -> bool {
        if self.first {
            self.first = false;
            return true;
        }

        self.ticks += 1;

        if self.ticks >= self.period {
            self.ticks -= self.period;
            true
        } else {
            false
        }
    }
}
