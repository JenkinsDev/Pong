// Axis-Aligned Bound Box Collision

pub trait CollisionDetection<T> {
    fn is_colliding(obj_1: &T, obj_2: &T) -> bool;
}

pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

pub struct BoundingBox {
    pub top_left: Vec2,
    pub top_right: Vec2,
    pub bottom_left: Vec2,
    pub bottom_right: Vec2
}

impl BoundingBox {
    pub fn translate_by(&mut self, x: f64, y: f64) {
        self.top_left = Vec2 { x: self.top_left.x + x, y: self.top_left.y + y };
        self.top_right = Vec2 { x: self.top_right.x + x, y: self.top_right.y + y };
        self.bottom_left = Vec2 { x: self.bottom_left.x + x, y: self.bottom_left.y + y };
        self.bottom_right = Vec2 { x: self.bottom_right.x + x, y: self.bottom_right.y + y };
    }

    pub fn translate_to(&mut self, x: f64, y: f64) {
        let width_of_object = self.top_right.x - self.top_left.x;
        let height_of_object = self.bottom_left.y - self.top_left.y;

        self.top_left = Vec2 { x: x, y: y };
        self.top_right = Vec2 { x: x + width_of_object, y: y };
        self.bottom_left = Vec2 { x: x, y: y + height_of_object };
        self.bottom_right = Vec2 {x: x + width_of_object, y: y + height_of_object };
    }
}

pub struct AABB;
impl CollisionDetection<BoundingBox> for AABB {
    fn is_colliding(obj_1: &BoundingBox, obj_2: &BoundingBox) -> bool {
        return obj_2.top_left.x < obj_1.top_right.x && obj_2.top_right.x > obj_1.top_left.x
          && obj_2.top_left.y < obj_1.bottom_left.y && obj_2.bottom_left.y > obj_1.top_left.y;
    }
}
