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
    

    
    
    
    



