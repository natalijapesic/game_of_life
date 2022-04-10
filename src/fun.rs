fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    //array array
    let operations: [[i8; 2]; 8] = [
        [0, 1],
        [0, -1],
        [1, 0],
        [-1, 0],
        [1, 1],
        [1, -1],
        [-1, 1],
        [-1, -1],
    ];

    
    let a: [i32; 5] = [1, 2, 3, 4, 5];


    //if in a let statement
    game.world[i][j] = if random == 0 { 1 } else { 0 };

    //repetition with loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; 
                //ovako se zove loop i kada dodje do 2 on izadje
            }
            remaining -= 1;
        }

        count += 1;
    }

    //loop vraca rezultat
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    //kao foreach
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    //countdown
    for number in (1..4).rev() {
        println!("{}!", number);
    }


}

fn tuples() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

//struct

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn strukture() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    //or
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

/*we want to borrow the struct rather than take ownership of it. 
This way, main retains its ownership and can continue using rect1, 
which is the reason we use the & in the function signature and 
where we call the function */
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/*Methods are similar to functions: we declare them with the fn keyword and a name, 
they can have parameters and a return value, and they contain some code that’s run 
when the method is called from somewhere else. Unlike functions, methods are defined
 within the context of a struct (or an enum or a trait object), 
and their first parameter is always self, which represents the instance
 of the struct the method is being called on. */

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        //bilo koja fja koja se nalazi na dalje
        //bice upucena strukturi Rectangle
        /*
        &self instead of rectangle: &Rectangle
        The &self is actually short for self: &Self
        the type Self is an alias for the type that the impl block is for.

        Methods can take ownership of self, self
        borrow self immutably, &self
        borrow self mutably, &mut self (change)
        just as they can any other parameter.

        this method borrows the Self instance
         */
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    
    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
    
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    //associated functions - najcesce kao konstruktor
    /*All functions defined within an impl block are called 
    associated functions because they’re associated with the 
    type named after the impl. We can define associated functions 
    that don’t have self as their first parameter */
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    fn main() {
        let sq = Rectangle::square(3);
    }

    //enum
    enum IpAddrKind {
        V4,
        V6,
    }
    
    fn main() {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
    
        route(IpAddrKind::V4);
        route(IpAddrKind::V6);
    }
    
    fn route(ip_kind: IpAddrKind) {}

    fn main() {
        enum IpAddr {
            V4(String),
            V6(String),
        }
    
        /*
        That is, IpAddr::V4() is a function call that takes 
        a String argument and returns an instance of the IpAddr 
        type. We automatically get this constructor function defined 
        as a result of defining the enum. 
        you can put any kind of data inside an enum 
        */

        let home = IpAddr::V4(String::from("127.0.0.1"));
    
        let loopback = IpAddr::V6(String::from("::1"));
    }
    
    //enum sa fjom
    fn main() {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
    
        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }
    
        let m = Message::Write(String::from("hello"));
        m.call();

        /*The body of the method would use self to 
        get the value that we called the method on. 
        In this example, we’ve created a variable m 
        that has the value Message::Write(String::from("hello")),
        and that is what self will be in the body of the call
         method when m.call() runs. */
    }


    /*
    The Option Enum and Its Advantages Over Null Values
    
    Rust doesn’t have the null feature
    */

    enum Option<T> {
        None,
        Some(T),
    }
    fn main() {
        let some_number = Some(5);
        let some_string = Some("a string");
    
        let absent_number: Option<i32> = None;
    }

    /*When we have a None value, in some sense, 
    it means the same thing as null: we don’t 
    have a valid value. 
    So why is having Option<T> any better than having null? */
    
//match + enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
/*catch-all arm last because the patterns are
 evaluated in order 
 */
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

/*Rust also has a pattern we can use when we don’t 
 want to use the value in the catch-all 
 pattern: _, which is a special pattern that 
 matches any value and does not bind to that value. 
 This tells Rust we aren’t going to use the value, 
 so Rust won’t warn us about an unused variable. */

 fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // ili ()
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
//isto
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}



//modules
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

//vektori
//kada izadje iz scope-a onda se brise sve
fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

//pristupanje elementima
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    /*
    panic ako ode van niza
     */
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    //Option<&>, zato je i match
    /*vraca none ako ode van niza, bez panic */
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

//iter
fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    /* dereferenciranje
    da bi pristupili vrednosti u i */ 
    for i in &mut v {
        *i += 50;
    }
}

/*when we need to store elements of a different type 
in a vector, we can define and use an enum! */
/*Rust needs to know what types will be in the 
vector at compile time so it knows exactly how 
much memory on the heap will be needed to store 
each element. */
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

//trings are implemented as a collection of bytes
    
fn main() {
    let mut s = String::new();
    let s = "initial contents".to_string();
    //ili 
    let s = String::from("initial contents");

    //konkateniranje
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push("bar");
}
   
//generic type

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        //ovo nece da radi za bilo koji tip
        if item > largest { 
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

//enum + generic

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

//generic + method
struct Point<T> {
    x: T,
    y: T,
}

/*posle impl mora da se def T 
kako bi znali da Point koristi generic type */
/*impl that declares the generic type will be 
defined on any instance of the type */
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

//trait
/*A trait tells the Rust compiler about 
functionality a particular type has and 
can share with other types. We can use 
traits to define shared behavior in 
an abstract way. */
pub trait Summary {
    fn summarize(&self) -> String;
}
/*declared the trait as pub so that crates 
depending on this crate can make use of this 
trait too */

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*The difference is that after impl, 
we put the trait name that we want 
to implement, then use the for keyword, 
and then specify the name of the type we 
want to implement the trait for. */

/*
1. moze da se definise kao interfejs
i da onda svaki tip koji implementira trait
da implementira taj trait na svoj nacin
---ali tip mora da implementira ako ne postoji
default imp

2. moze da postoji default implementacija
i da onda svaki tip ima istu..kao virtual metode
ali takodje tip moze da override-uje tu fju

3.default imp mogu da zovu druge metode unutar trait-a

4.trait moze biti i parametar neke fje
time se kaze fji da parametar moze biti bilo sta
sto implementira dati trait*/

//ovo moze lepse da se napise pomocu Where
pub fn notify<T: Summary + Display + Clone>(item1: &T, item2: &T){
    println!("Breaking news! {}", item.summarize());
}

//closures
// closures are able to capture their evnironment
/*
in 3 ways:
1. taking ownership FnOnce, koristi se move
- ovaj closure se zove samo jednom
2. borrowing immutably Fn
3. borrowing mutably FnMut
 */
/*razlika izmedju fje i closure-a 
je sto se parametri nalaze unutar ||
ako fja ima jednu liniju onda nam ne treba {}

var ne cuva rezultat fje vec sam closure
ne mora da se definise tip parametara niti 
povratni tip*/

fn main(){

    //funcion def
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }

    //fully annotated closure def
    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    //closure skraceno
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

/* Fn trait, iz std i svi closures implementiraju
3 fn-a: Fn, FnMut, FnOnce 
kao i obicne fje

new je contructor*/

/*************
 * 
 * 
 * Optional koristim kao type kada npr 
 * pravim neki strukt koji na pocetku ima
 * None vrednost a kasnije se inicijalizuje
*/


/* iterators */
//All iterators implement a trait 
///named Iterator from std
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

pub trait Iterator {
    type Item;
    //Item, povratni tip Iterator-a
    /*next metoda vraca sledeci element iz niza
    vrapovan u Some i kada se dodje do kraja
    on vraca None*/
    fn next(&mut self) -> Option<Self::Item>;
}

/*
v.iter(), immutable references to the values in the vector
v.iter_mut(), mutable references
v.into_iter() iterator takes ownership of v and returns
owned values
 */

/***************
 * assert_eq!.. verovatno poredi dva elementa?
 */

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}


//testovi

/*
Set up any needed data or state.
Run the code you want to test.
Assert the results are what you expect.

 To change a function into a test function, add #[test]
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/*
nije bitno sta je prvi a sta drugi argument
assert_eq! za jednakost
assert_ne! za nejednakost

We place the #[should_panic] attribute after the #[test] attribute 
and before the test function it applies to.

To make should_panic tests more precise, we can add an optional 
expected parameter to the should_panic attribute.
*/

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //expected se dodaje kako bi se preciznije odredilo zasto je 
    //doslo do panic result-a
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}


//umesto da panici on samo detektuje gresku
//ovde ne moze da se koristi #[should_panic]
//sto je i logicno jer ovde baca error a ne panic
// upitnik se koristi na kraju i tada se ocekuje Result
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

//kada postoje vise testova, oni se izvrsavaju paralelno
//$ cargo test -- --test-threads=1
//za definisanje broja niti 
//ako u funkciji koju pozivamo imamo println onda ce taj println da se istampa
//samo ako fail-uje test


//ali ako zelim da vidim rezultat vrednosti za testove koji su prosli
//onda: $ cargo test -- --show-output


//ako zelim da pozovem samo jedan test onda je dovoljno da upisem
//cargo test ime_test_funkcije

//We can specify part of a test name, 
//and any test whose name matches that value will be run

#[test]
#[ignore] //za vece testove, koji trenutno nisu potrebni
fn expensive_test() {
    // code that takes an hour to run
}
//ako zelim samo njega da aktiviram
//cargo test -- --ignored

//unit testovi, nalaze se u okviru fajla
//gde se i funkcije koje treba testirati nalaze

//integration testovi se nalaze u posebnom fajlu
// Their purpose is to test whether many parts of your library work together correctly
//nalaze se u dir tests odmah do src-a
/*
We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)].
Cargo treats the tests directory specially and compiles files in this directory only
when we run cargo test */

//ako zelim da pokrenem samo integration testove:
//cargo test --test integration_test

/***************************** */
/*Treating each integration test file as its own crate is useful to create 
separate scopes that are more like the way end users will be using your crate.
However, this means files in the tests directory don’t share the same behavior
as files in src do, as you learned in Chapter 7 regarding how to separate code
into modules and files. */



