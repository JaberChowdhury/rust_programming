enum Discount {
    Percent(i32),
    Flat(i32),
}
// struct Ticker {
//     event: String,
//     price: i32,
// }

fn print_result(discount: Discount) {
    match discount {
        Discount::Flat(21) => println!("good discount"),
        Discount::Flat(12) => println!("Awesome"),
        Discount::Percent(12) => println!("Maxxing??"),
        _ => println!("Nice"),
    }
}
// 1. Define the inner enum first
enum IngredientType {
    Flour(i32),
    Water(i32),
}

// 2. Use it inside the outer enum
enum FlourTypeRecipe {
    Name(String), // Usually a Name variant holds a String
    // Correct: VariantName(InnerEnumType)
    Ingredient(IngredientType),
}
fn main() {
    print_result(Discount::Flat(12));
    print_result(Discount::Percent(12));

    // Create an instance of the inner enum
    let flour = IngredientType::Flour(500);

    // Create the outer enum containing the inner one
    let recipe = FlourTypeRecipe::Ingredient(flour);

    // Matching requires nested match or if-let
    match recipe {
        FlourTypeRecipe::Name(n) => println!("Recipe: {}", n),
        FlourTypeRecipe::Ingredient(inner) => match inner {
            IngredientType::Flour(amount) => println!("Flour: {}", amount),
            IngredientType::Water(amount) => println!("Water: {}", amount),
        },
    }

    // Or using pattern matching directly in one go:
    // if let FlourTypeRecipe::Ingredient(IngredientType::Flour(amount)) = recipe {
    //     println!("Direct match: {}g flour", amount);
    // }
}
