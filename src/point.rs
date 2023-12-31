
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }

    pub fn distance(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn distance_squared(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        dx * dx + dy * dy + dz * dz
    }

    pub fn dot(&self, other: &Point) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Point) -> Point {
        Point {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.z,
        }
    }

    pub fn length(&self) -> f32 {
        self.distance(&Point::default())
    }

    pub fn length_squared(&self) -> f32 {
        self.distance_squared(&Point::default())
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Point {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn scale(&mut self, scalar: f32) -> Self {
            self.x *= scalar;
            self.y *= scalar;
            self.z *= scalar;
            *self
    }

}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

