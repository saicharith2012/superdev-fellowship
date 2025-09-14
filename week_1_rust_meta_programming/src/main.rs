// computes an arithmetic expression
macro_rules! eval {
    ($expr: expr) => {
        $expr
    };
}

// macro to create a vector
macro_rules! vector {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let ans = eval!(2 + 3 * 5);
    println!("{}", ans);

	let ans2 = vector!(1, 2, 3);
	println!("{}", ans2[0]);
}

// -----------------------------------------------------------------------------

// #[derive(Debug)] // Debug is a custom derive procedural macro
// struct User {
//     name: String,
//     age: u32
// }

// fn main() {
//     println!("hi there"); // declarative macro
// }

//#[post("/user/")] // attribute like procedural macro
// fn create_user() {
//    sqlx::query!("INSERT INTO USERS VALUES ()") // function like procedural macro
// }
// -------------------------------------------------------------------------

// #[derive(Debug)] // derived macros are imported like this.

// struct User {
//     name: String,
//     age: u32,
// }

// // impl std::fmt::Display for User {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         write!(f, "({}, {})", self.name, self.age)
// //     }
// // }

// // impl std::fmt::Debug for User {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         write!(f, "({}, {})", self.name, self.age)
// //     }
// // }

// fn main() {
//     let u = User {
//         name: String::from("charith"),
//         age: 22,
//     };

//     // println!("{}", u);
//     println!("{:?}", u);
// }

// -------------------------------------------------------------------

// pub trait Shape {
//     fn shape_name() -> String; // associated static function per type
//     fn area(&self) -> f32;
//     fn perimeter(&self) -> f32;
// }

// struct Rect {
//     width: f32,
//     height: f32,
// }

// impl Shape for Rect {
//     fn area(&self) -> f32 {
//         self.width * self.height
//     }
//     fn perimeter(&self) -> f32 {
//         2.0 * (self.width + self.height)
//     }
//     fn shape_name() -> String {
//         String::from("Rectangle")
//     }
// }

// struct Square {
//     side: f32,
// }

// impl Shape for Square {
//     fn area(&self) -> f32 {
//         self.side * self.side
//     }
//     fn perimeter(&self) -> f32 {
//         4.0 * self.side
//     }
//     fn shape_name() -> String {
//         String::from("Square")
//     }
// }

// fn main() {
//     println!("Hello, world!");

//     let rect1 = Rect {
//         width: 2.0,
//         height: 4.0,
//     };

//     let square1 = Square { side: 5.0 };

//     print_area_perimeter(rect1);
//     print_area_perimeter(square1);
// }

// fn print_area_perimeter<T: Shape>(s: T) {
//     println!("{} - area: {}, perimeter: {}", T::shape_name(), s.area(), s.perimeter())
// }
