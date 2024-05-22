const PI: f64 = 3.141592;

fn main() {
    println!("The password for AUT's Math Faculty WiFi is: {PI}");

    let age = 13;
    println!("A pretty young lad was {age} years old...");
    {
        let age = 23;
        println!("But he grew to the age of {age} -- pretty old!");
    }
    println!("Drank from the fountainhead waters... now he is {age} again!");
}
