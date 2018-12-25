use crate::plane::Plane;

pub struct Map {
   pub width: u16,
   pub height: u16,
   pub name: String,

   pub plane1: Plane,
   pub plane2: Plane
}
