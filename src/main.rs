use chrono::{Local, Datelike};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let name = match inquire::Text::new("What is your name?").prompt() {
        Ok(name) => name,
        Err(err) => return Err(Box::new(err))
    };

    println!("Hello {}!", name);

    let age = match inquire::DateSelect::new("When were you born?").prompt() {
        Ok(dob) => {
            let today = Local::now().naive_local().date();
            let mut age = today.year() - dob.year();
            if (today.month(), today.day()) < (dob.month(), dob.day()) {
                age -= 1;
            }
            age
        },
        Err(err) => {
            println!("Error: {}", err);
            0
        }
    };

    println!("You are {} years old", age);

    // uk cities
    let cities = vec!["London", "Birmingham", "Manchester", "Glasgow", "Newcastle", "Sheffield", "Liverpool", "Leeds", "Bristol", "Edinburgh", "Belfast", "Cardiff", "Nottingham", "Leicester", "Sunderland", "Brighton", "Hull", "Plymouth", "Stoke", "Wolverhampton", "Derby", "Swansea", "Southampton", "Aberdeen", "Portsmouth", "York", "Peterborough", "Dundee", "Lancaster", "Oxford", "Cambridge", "Canterbury", "Chichester", "Winchester", "Lichfield", "Worcester", "Hereford", "Exeter", "Salisbury", "Truro", "St Albans", "Ripon", "Wakefield", "Wells", "Ely", "Carlisle", "Chester", "Durham", "Gloucester", "Lincoln", "Norwich", "Preston", "Salford", "Stirling", "Wells", "Worcester", "York"];

    let birth_place = inquire::Select::new("Where were you born?", cities.clone()).prompt().unwrap_or_else(|err| {
        println!("Error: {}", err);
        "Unknown"
    });

    println!("You were born in {}", birth_place);

    let favourite_cities = inquire::MultiSelect::new("What are your favourite cities?", cities).prompt().unwrap_or_else(|err| {
        println!("Error: {}", err);
        vec![]
    });

    println!("Your favourite cities are: {:?}", favourite_cities);

    let net_worth = inquire::CustomType::<f64>::new("What is your net worth?")
        .with_formatter(&|value| format!("£{:.2}", value))
        .with_error_message("You must enter a valid value")
        .with_help_message("Enter a number in pounds")
        .prompt().unwrap_or(0.0);

    println!("Your net worth is {:.2}", net_worth);

    Ok(())
}
