#![allow(unused)]

use crate::prelude::*;


pub struct ColorBox {
    colors: Vec<Color>,
}

impl ColorBox {
    pub fn new(color_list: Vec<Color>) -> Self {
        Self { 
            colors: color_list.to_owned() 
        }
    }
    pub fn new_with_colors() -> Self {
        Self { 
            colors: vec![
                Color::CYAN, Color::GREEN, Color::RED,
                Color::YELLOW, Color::LIME_GREEN, Color::ORANGE_RED,
                Color::SILVER, Color::PINK, Color::AZURE, 
                Color::YELLOW_GREEN, Color::DARK_GREEN, Color::BLUE, 
                Color::TURQUOISE, Color::ALICE_BLUE 
            ] 
        }
    }
    pub fn choose_color(&self) -> Color {
        let mut rnd = thread_rng();
        let num = self.colors.len();
        let c = rnd.gen_range(0..num);
        return self.colors[c];
    }
    pub fn choose_color_from_count(&self, mut count: usize) -> Color {
        let mut rnd = thread_rng();
        let num = self.colors.len();
        if num < count {count = num};
        let c = rnd.gen_range(0..count);
        return self.colors[c];
    }

}