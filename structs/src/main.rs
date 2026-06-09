struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); // move

    let use3 = build_user("ja@gmai".to_string(), "da".to_string());

    // Creating Instances with Struct Update Syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user2 // move
    };

    // Creating Different Types with Tuple Structs

    //Defining Unit-Like Structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // Example
    let width = 32;
    let height = 43;

    let rect = (21, 34);
    let mut area = calculate_area_val(width, height);
    area = calculate_area_tuple(rect);
    let rec = rectangle {
        width: 32,
        height: 1,
    };

    area = calculate_area_struct(&rec);

    println!(" Area of {:#? } -> {}", rec, area);

    // method
    let rec = Rectangle {
        width: 32,
        height: 1323,
    };

      let rec1 = Rectangle {
        width: 33,
        height: 1343,
    };

    println!(" Area of {}", Rectangle::square(4).area());
}
#[derive(Debug)]
struct rectangle {
    width: u32,
    height: u32,
}

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

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn calculate_area_val(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_area_tuple(dim: (u32, u32)) -> u32 {
    let (width, height) = dim;
    width * height
}

fn calculate_area_struct(dim: &rectangle) -> u32 {
    dim.width * dim.height
}
