use std::fmt;

// Part 1: Define Pokémon
// 1. Enum type for Pokémon types
#[derive(Debug, Clone, PartialEq)]
enum PokemonType {
    Fire,
    Water,
    Grass,
    Electric,
    Normal,
}

impl fmt::Display for PokemonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PokemonType::Fire => write!(f, "Fire"),
            PokemonType::Water => write!(f, "Water"),
            PokemonType::Grass => write!(f, "Grass"),
            PokemonType::Electric => write!(f, "Electric"),
            PokemonType::Normal => write!(f, "Normal"),
        }
    }
}

// Enum for gender
#[derive(Debug, Clone, PartialEq)]
enum Gender {
    Male,
    Female,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
        }
    }
}

// 2. Pokémon structure
#[derive(Debug, Clone)]
struct Pokemon {
    name: String,
    level: u32,
    pokemon_type: PokemonType,
    experience: u32,
    gender: Gender,
}

// Part 2: Functions and behaviors
impl Pokemon {
    // Constructor
    fn new(name: &str, pokemon_type: PokemonType, gender: Gender) -> Self {
        Pokemon {
            name: String::from(name),
            level: 1,
            pokemon_type,
            experience: 0,
            gender,
        }
    }

    // Gain experience
    fn gain_xp(&mut self, xp: u32) {
        self.experience += xp;

        // Check if the Pokémon can level up
        while self.experience >= 100 {
            self.level += 1;
            self.experience -= 100;
            println!("{} leveled up to level {}!", self.name, self.level);
        }
    }

    // Display Pokémon information
    fn display(&self) {
        println!("Name: {}", self.name);
        println!("Type: {}", self.pokemon_type);
        println!("Level: {}", self.level);
        println!("Experience: {}/100", self.experience);
        println!("Gender: {}", self.gender);
        println!("-----------------------");
    }

    // Check if two Pokémon can breed
    fn can_breed_with(&self, other: &Pokemon) -> bool {
        // Same type, opposite genders, and sufficient levels (arbitrarily set to 5)
        self.pokemon_type == other.pokemon_type
            && ((self.gender == Gender::Male && other.gender == Gender::Female)
            || (self.gender == Gender::Female && other.gender == Gender::Male))
            && self.level >= 5
            && other.level >= 5
    }

    // Part 3: Breeding
    fn breed_with(&self, other: &Pokemon) -> Option<Pokemon> {
        if self.can_breed_with(other) {
            // Create a new Pokémon at level 1
            let baby = Pokemon {
                name: String::from("Mystery"),
                level: 1,
                pokemon_type: self.pokemon_type.clone(),
                experience: 0,
                // For bonus: random gender
                gender: if rand::random::<bool>() {
                    Gender::Male
                } else {
                    Gender::Female
                },
            };

            println!("An egg hatched! A new {} Pokémon was born!", baby.pokemon_type);
            Some(baby)
        } else {
            println!("These Pokémon cannot breed together.");
            None
        }
    }
}

// Part 4: Breeding management
struct Breeding {
    pokemon: Vec<Pokemon>,
}

impl Breeding {
    fn new() -> Self {
        Breeding {
            pokemon: Vec::new(),
        }
    }

    // Add a Pokémon
    fn add_pokemon(&mut self, pokemon: Pokemon) {
        println!("{} was added to the breeding center!", pokemon.name);
        self.pokemon.push(pokemon);
    }

    // Display all Pokémon
    fn display_all(&self) {
        println!("\n=== LIST OF POKÉMON IN THE BREEDING CENTER ({}) ===", self.pokemon.len());
        if self.pokemon.is_empty() {
            println!("The breeding center is empty!");
        } else {
            for (index, pokemon) in self.pokemon.iter().enumerate() {
                println!("Pokémon #{}", index + 1);
                pokemon.display();
            }
        }
    }

    // Train all Pokémon
    fn train_all(&mut self, xp: u32) {
        println!("\n=== TRAINING ALL POKÉMON ===");
        for pokemon in &mut self.pokemon {
            println!("{} gains {} XP!", pokemon.name, xp);
            pokemon.gain_xp(xp);
        }
    }

    // Attempt breeding between two Pokémon
    fn attempt_breeding(&mut self, index1: usize, index2: usize) -> bool {
        if index1 >= self.pokemon.len() || index2 >= self.pokemon.len() {
            println!("Invalid Pokémon index!");
            return false;
        }

        // To avoid borrowing issues, we temporarily clone
        let pokemon1 = self.pokemon[index1].clone();
        let pokemon2 = self.pokemon[index2].clone();

        println!("\n=== BREEDING ATTEMPT ===");
        println!("Between {} and {}", pokemon1.name, pokemon2.name);

        if let Some(baby) = pokemon1.breed_with(&pokemon2) {
            self.add_pokemon(baby);
            true
        } else {
            false
        }
    }

    // Bonus: Sort Pokémon by level
    fn sort_by_level(&mut self) {
        self.pokemon.sort_by(|a, b| b.level.cmp(&a.level));
        println!("Pokémon sorted by level!");
    }

    // Bonus: Sort Pokémon by type
    fn sort_by_type(&mut self) {
        self.pokemon.sort_by(|a, b| {
            let type_a = format!("{}", a.pokemon_type);
            let type_b = format!("{}", b.pokemon_type);
            type_a.cmp(&type_b)
        });
        println!("Pokémon sorted by type!");
    }
}

// Main function for testing
use std::io;

fn main() {
    // Create the breeding center
    let mut breeding = Breeding::new();

    // Create some Pokémon
    let pikachu = Pokemon::new("Pikachu", PokemonType::Electric, Gender::Male);
    let raichu = Pokemon::new("Raichu", PokemonType::Electric, Gender::Female);
    let charizard = Pokemon::new("Charizard", PokemonType::Fire, Gender::Male);
    let venusaur = Pokemon::new("Venusaur", PokemonType::Grass, Gender::Female);

    // Add Pokémon to the breeding center
    breeding.add_pokemon(pikachu);
    breeding.add_pokemon(raichu);
    breeding.add_pokemon(charizard);
    breeding.add_pokemon(venusaur);

    loop {
        println!("\n=== Breeding Center Menu ===");
        println!("1. Display all Pokémon");
        println!("2. Train all Pokémon");
        println!("3. Attempt breeding");
        println!("4. Sort Pokémon by level");
        println!("5. Sort Pokémon by type");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim().parse::<u32>();

        match choice {
            Ok(1) => breeding.display_all(),
            Ok(2) => {
                println!("Enter XP to train all Pokémon:");
                let mut xp_input = String::new();
                io::stdin().read_line(&mut xp_input).expect("Failed to read input");
                if let Ok(xp) = xp_input.trim().parse::<u32>() {
                    breeding.train_all(xp);
                } else {
                    println!("Invalid XP value!");
                }
            }
            Ok(3) => {
                println!("Enter the indices of two Pokémon to breed (e.g., 0 1):");
                let mut indices = String::new();
                io::stdin().read_line(&mut indices).expect("Failed to read input");
                let indices: Vec<usize> = indices
                    .trim()
                    .split_whitespace()
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
                if indices.len() == 2 {
                    breeding.attempt_breeding(indices[0], indices[1]);
                } else {
                    println!("Invalid indices!");
                }
            }
            Ok(4) => breeding.sort_by_level(),
            Ok(5) => breeding.sort_by_type(),
            Ok(6) => {
                println!("Exiting the breeding center. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }
}