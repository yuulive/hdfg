pub const EGGS_MIN: i32 = 1;
pub const FLOUR_MIN: f32 = 100.0;
pub const MILK_MIN: f32 = 200.0;
pub fn find_blin_amount(mut flour_amount: f32, mut milk_amount: f32, mut eggs_amount: i32) -> f32 {
    flour_amount = flour_amount / FLOUR_MIN;
    eggs_amount = eggs_amount / EGGS_MIN;
    milk_amount = milk_amount / MILK_MIN;
    let smallest: f32;
    if flour_amount <= milk_amount && flour_amount <= eggs_amount as f32{
        smallest = flour_amount as f32;
        return smallest * 6.0;
    }
    else if milk_amount <= flour_amount && milk_amount <= eggs_amount as f32 {
        smallest = milk_amount as f32;
        return smallest * 6.0;
    }
    else if eggs_amount as f32 <= flour_amount && eggs_amount as f32 <= milk_amount {
        smallest = eggs_amount as f32;
        return smallest as f32 * 6.0;
    }
    else{
        return -1 as f32;
    }
}
pub fn find_materials_amount(mut flour_amount: f32, mut milk_amount: f32, mut eggs_amount: i32) -> (f32,f32,i32) {
    flour_amount = flour_amount / FLOUR_MIN;
    eggs_amount = eggs_amount / EGGS_MIN;
    milk_amount = milk_amount / MILK_MIN;
    let mut smallest: f32 = 0.0;
    if flour_amount<=milk_amount && flour_amount<=eggs_amount as f32 {
        smallest = flour_amount as f32;
    }
    else if milk_amount<=flour_amount && milk_amount<=eggs_amount as f32 {
        smallest = milk_amount as f32;
    }
    else if eggs_amount as f32<=flour_amount && eggs_amount as f32<=milk_amount {
        smallest = eggs_amount as f32;
    }
    (smallest * FLOUR_MIN, smallest * MILK_MIN, smallest as i32 * EGGS_MIN)
}
