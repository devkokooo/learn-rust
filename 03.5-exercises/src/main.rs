// Exercises to practice:
// - variables
// - scalar and compound data types
// - functions
// - comments
// - `if` expressions
// - loops
fn main() {
    println!("Converting Fahrenheit to Celsius...");
    let temp: i16 = 72;
    let celsius: f32 = fahrenheit_to_celsius(temp);
    println!("{temp}F is {celsius}C\n");

    println!("Calculating the 7th Fibonacci number...");
    let ans = fib(7);
    println!("The answer is {ans}\n");

    println!("Printing the Christmas Carol lyrics...");
    print_christmas_carol_lyrics();
}

// Convert temperatures between Fahrenheit and Celsius
fn fahrenheit_to_celsius(temperature: i16) -> f32 {
    // C = (F - 32) * 5/9
    ((temperature - 32) * 5/9).into()
}

// Generate the nth Fibonacci number
fn fib(mut n: i16) -> i16 {
    // Fn = Fn-1 + Fn-2
    // F0 = 1, F1 = 1
    let mut f0: i16 = 1;
    let mut f1: i16 = 1;

    // F2 = F1 + F0 = 1 + 1 = 2
    // F3 = F2 + F1 = 2 + 1 = 3
    while n > 0 {
        println!("n:{n} f0:{f0} f1:{f1}");
        let f2 = f0 + f1;
        f0 = f1;
        f1 = f2;
        n -= 1;
    }

    f0
}

// Print the lyrics to the Christmas carol "The Twelve Days of Christmas", taking
// advantage of the repetition in the song
fn print_christmas_carol_lyrics() {
    let mut n = 1;

    let num_to_day = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];
    let gifts = [
        "A partridge in a pair tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swams a swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    while n <= 12 {
        println!("On the {} day of Christmas, my true love sent to me", num_to_day[n - 1]);

        for num in (1..n + 1).rev() {
            println!("{}", gifts[num - 1]);
        }
        println!("");
        n += 1;
    }
}
