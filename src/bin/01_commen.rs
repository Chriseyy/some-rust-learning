

// Variables and Mutablility

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// ohne let mut können wir den Wert von x nicht ändern, da er standardmäßig unveränderlich ist. 
// Wenn wir versuchen würden, x ohne mut zu ändern, würde der Compiler einen Fehler anzeigen.