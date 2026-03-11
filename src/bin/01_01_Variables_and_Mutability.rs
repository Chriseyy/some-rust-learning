

// to declare a variable, we use the let keyword, followed by the variable name and an optional type annotation.
// The syntax for declaring a variable is as follows:
// let variable_name: type = value; 

fn start_idea() {
    let x = 5; // this is an immutable variable, we cannot change its value
    println!("The value of x is: {x}");

    let mut y = 10; // this is a mutable variable, we can change its value
    println!("The value of y is: {y}");
    y = 15; // we can change the value of y because it is mutable
    println!("The value of y is: {y}");

    let z: i32 = 20; // this is a variable with a type annotation, we can change its value and shows how big the variable is in memory, in this case 4 bytes because i32 is a 32-bit integer
    println!("The value of z is: {z}");
    // z = 25; // this will cause an error because z is immutable
    // you can also declare a variable without initializing it, but you must specify the type annotation in this case
    let a: i32; // this is a variable declaration without initialization
    a = 30; // we can initialize the variable later
    println!("The value of a is: {a}");
}

// Variables and Mutablility

fn variab_mut() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// ohne let mut können wir den Wert von x nicht ändern, da er standardmäßig unveränderlich ist. 
// Wenn wir versuchen würden, x ohne mut zu ändern, würde der Compiler einen Fehler anzeigen.

// -----------------------

// Declaring Constants

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
const MAX_POINTS: u32 = 100_000;

fn declaing_const() {
    {
        let x = THREE_HOURS_IN_SECONDS + MAX_POINTS;
        println!("The value of x: {x}");
    }
}

// Konstanten müssen immer einen Typ haben und können nicht von einem Ausdruck abgeleitet werden.
// Konstanten sind immer unveränderlich und können nicht mit mut deklariert werden.
// Konstanten können auch in einem globalen Kontext deklariert werden, 
// während Variablen nur innerhalb von Funktionen deklariert werden können.


// -----------------------

// Shadowing

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// Shadowing ermöglicht es uns, eine neue Variable mit demselben Namen wie eine vorherige Variable zu deklarieren.
// In diesem Beispiel wird die Variable x zuerst mit dem Wert 5 deklariert.
// Dann wird eine neue Variable x deklariert, die den Wert der vorherigen Variable x plus 1 hat, also 6.
// Innerhalb des inneren Blocks wird eine weitere Variable x deklariert, die den Wert der vorherigen Variable x multipliziert mit 2 hat, also 12.

// Die geschweiften Klammern { } erschaffen in Rust einen abgetrennten "Raum" (Scope).
// Du hast dein äußeres x (Wert 6).
// Du gehst in den Raum { und erstellst mit let x ein zweites, inneres x (Wert 12).
// Solange du in diesem Raum bist, verdeckt das innere x das äußere. println! sieht also nur die 12.
// Sobald der Raum mit } geschlossen wird, wird das innere x (die 12) gelöscht und zerstört.
// Schwupps! Das äußere x (die 6) tritt wieder aus dem Schatten hervor und ist wieder gültig.

fn shadowing2() {
    // ohne 
    let eingabe_string = "42";
    let eingabe_getrimmt = eingabe_string.trim();
    let eingabe_zahl: i32 = eingabe_getrimmt.parse().unwrap();
    println!("Die Zahl ist ohne Shadowing: {eingabe_zahl}");

    let eingabe = "42";
    let eingabe = eingabe.trim();
    let eingabe: i32 = eingabe.parse().unwrap();
    println!("Die Zahl ist mit Shadowing: {eingabe}");

    // locking down:
    // let mut passwort = hole_passwort_vom_user();
    let mut passwort = "geheim";
    println!("Das Passwort ist: {passwort}");
    // verschlüsseln in praxis 
    // passwort.verschlüsseln();

    // let passwort = passwort;
    let passwort = "passwort123";
    // damit ist passwort jetzt unveränderlich, und wir können es nicht mehr versehentlich ändern.
    println!("Das Passwort ist: {passwort}");

    // entfernt soszugen das mut, aber wir können immer noch den Wert von passwort ändern, 
    // indem wir eine neue Variable mit demselben Namen deklarieren.
}





fn main() {
    start_idea();
    variab_mut();
    declaing_const();
    shadowing();
    shadowing2();
}