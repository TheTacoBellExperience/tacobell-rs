extern crate rand;

use rand::Rng;

const COUNTS : &'static [&'static str] =
    &["Double",
      "Triple",
      "Quadruple"];

const ADJECTIVES : &'static [&'static str] =
    &["Beefy",
      "Cheesy",
      "Spicy",
      "Fiery",
      "Crunchy",
      "Crispy",
      "Loaded",
      "Grilled",
      "Smothered",
      "Stuft",
      "Cantina"];

const FILLERS : &'static [&'static str] =
    &["Potato",
      "Nacho Cheese",
      "Bean",
      "Black Bean",
      "Rice",
      "Ground Beef",
      "Shredded Chicken",
      "Chicken",
      "Steak",
      "Fajitas",
      "Fritos®",
      "Doritos® Locos",
      "Fiery Doritos® Locos",
      "Cool Ranch® Doritos® Locos",
      "Nacho Cheese Doritos® Locos"];

const MEAL_MODIFIERS : &'static [&'static str] =
    &["Fiesta",
      "Fresco",
      "Fresco Grilled",
      "Lava"];

const MEALS : &'static [&'static str] =
    &["Taco",
      "Soft Taco",
      "Double Decker Taco",
      "Taco Salad",
      "Burrito",
      "Gordita",
      "Chalupa",
      "Crunchwrap",
      "Quesadilla",
      "Griller",
      "Mexican Pizza",
      "Quesarito",
      "Crunchwrap Slider",
      "Tostada",
      "Meximelt®",
      "XXL Grilled Stuft Burrito",
      "Smothered Burrito",
      "Combo Burrito",
      "5-Layer Burrito",
      "7-Layer Burrito",
      "Nachos",
      "Nachos Bellgrande®",
      "Doritos® Locos Taco",
      "Doritos® Locos Gordita",
      "Doritos® Locos Chalupa",
      "Doritos® Locos Nachos",
      "Waffle Taco",
      "Enchirito",
      "Roll-Up",
      "Power Bowl"];

const MODIFIERS : &'static [&'static str] =
    &["Crunch",
      "Supreme®",
      "Party Pack"];

fn get_phrase<R: Rng>(list: &'static [&'static str], chance : f32, rng: &mut R) -> Option<&'static str> {
    if rng.next_f32() < chance {
        Some(rng.choose(list).unwrap())
    } else {
        None
    }
}

/// Create a food item in the style of Taco Bell
///
/// This uses several lists of "food components" that are found in
/// actual Taco Bell menu items to create new foods that nobody would
/// ever expect.
pub fn generate() -> String {
    let mut food : Vec<Option<&'static str>> = vec![];
    let mut rng = rand::thread_rng();
    food.push(get_phrase(COUNTS, 0.1, &mut rng));
    food.push(get_phrase(ADJECTIVES, 0.85, &mut rng));
    food.push(get_phrase(FILLERS, 0.75, &mut rng));
    food.push(get_phrase(MEAL_MODIFIERS, 0.1, &mut rng));
    food.push(get_phrase(MEALS, 1.0, &mut rng));
    food.push(get_phrase(MODIFIERS, 0.25, &mut rng));
    let filtered : Vec<&'static str> = food.iter().filter_map(|x| x.clone()).collect();
    filtered.join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gen_string() {
        println!("{}", generate());
    }
}
