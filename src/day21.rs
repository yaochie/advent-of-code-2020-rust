use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

pub fn day21() {
    let mut all_ingredients = HashSet::new();

    let mut foods = Vec::new();
    let mut ingredient_count: HashMap<String, i32> = HashMap::new();
    let mut non_allergen: HashMap<String, HashSet<String>> = HashMap::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                let tokens: Vec<_> = line.trim().split_whitespace().collect();

                let ingredients: HashSet<String> = tokens
                    .iter()
                    .take_while(|token| token.chars().nth(0).unwrap() != '(')
                    .map(|s| s.to_string())
                    .collect();

                let allergens: HashSet<String> = tokens
                    .iter()
                    .skip_while(|token| token.chars().nth(0).unwrap() != '(')
                    .map(|s| s[..(s.len() - 1)].to_string())
                    .skip(1)
                    .collect();

                for ingredient in &ingredients {
                    if ingredient_count.contains_key(ingredient) {
                        *(ingredient_count.get_mut(ingredient).unwrap()) += 1;
                    } else {
                        ingredient_count.insert(ingredient.clone(), 1);
                    }
                }

                for allergen in &allergens {
                    if !non_allergen.contains_key(allergen) {
                        non_allergen.insert(allergen.clone(), HashSet::new());
                    }
                }

                all_ingredients.extend(ingredients.clone());
                foods.push((ingredients, allergens));
            }
        }
    }

    for (ingredients, allergens) in &foods {
        // all other ingredients cannot contain these allergens
        for ingredient in all_ingredients.difference(ingredients) {
            for allergen in allergens {
                non_allergen
                    .get_mut(allergen)
                    .unwrap()
                    .insert(ingredient.clone());
            }
        }
    }

    // find ingredients that cannot contain any allergens
    let mut non_allergen_ingredients = all_ingredients.clone();
    for ingredients in non_allergen.values() {
        non_allergen_ingredients.retain(|i| ingredients.contains(i));
    }

    let mut answer = 0;
    for ingredient in &non_allergen_ingredients {
        let count = ingredient_count.get(ingredient).unwrap();
        answer += count;
    }
    println!("Part A answer: {}", answer);

    println!();
    println!("Part B:");
    println!("Possible ingredients:");
    
    // do manually??
    for (allergen, ingredients) in non_allergen.iter() {
        println!("{}, {}", allergen, ingredients.len());
        let x: HashSet<_> = all_ingredients.difference(ingredients).collect();
        println!("{:?}", x);
    }

    /*
    manual answer:
    dairy	qqskn
    eggs	ccvnlbp
    fish	tcm
    nuts	jnqcd
    peanuts	qjqb
    sesame	xjqd
    shellfish	xhzr
    soy	cjxv
    */
}
