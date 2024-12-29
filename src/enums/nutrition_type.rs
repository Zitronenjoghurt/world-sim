#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum NutritionType {
    Plant = 0,
    Meat = 1,
}