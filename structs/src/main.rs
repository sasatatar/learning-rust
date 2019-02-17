fn main() {
    let user1 = create_new_user("john@smith.com".to_string(), String::from("John"));
    println!("New user created: {}, ({})", user1.name, user1.email);

    let user2 = User {
        name: "Jane".to_string(),
        ..user1
    };

    println!("User 1: {}", user1.name);
    println!("User 2: {}", user2.name);
    // this will not work since we moved the ownership of user1.email to user2.email, so we cannot access it any more through user1!
    // println!("User 1 email: {}", user1.email);
    // this works fine
    println!("User 2 email: {}", user2.email);

    // tuple struct
    let color1 = Color(21, 244, 43);
    log_color(&color1);
    println!("Red: {}, Green: {}, Blue: {}", color1.0, color1.1, color1.2);

    let rect = Rectangle { 
        width: 20,
        height: 50
    };

    println!("The rect area is: {} px.", area(&rect));
    println!("The rect area is: {} px.", rect.area());

    println!("rect is {:#?}", rect);

    // methods with more parameters
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // associated functions
    let sq = Rectangle::square(3);
    println!("The sq area is: {}", sq.area());
}

fn create_new_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1
    }
}

fn log_color(color: &Color) {
    println!("R: {}", color.0);
    println!("G: {}", color.1);
    println!("B: {}", color.2);    
}

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
