
// fn main() {
//     let mut shopping_list: Vec<&str> = Vec::new();
//     shopping_list.push("milk");
// }

// Bài 2: Trường dữ liệu value của Wrapper có thể sử dụng u32 hoặc String hoặc …
// struct Wrapper <T> {
//     value: T,
// }
// impl<T> Wrapper<T> {
//     pub fn new (value: T) -> Self {
//         Wrapper { value }
//     }
// }

// Bài 3: Bước 1: Thực hiện một implement in ra tất cả các trường dữ liệu.
//  Bước 2: đối với trường dữ liệu grade, ta có thể có trường hợp là “A+”, “B+” ,...
// #[derive(Debug)]
// fn main() {
//     let report_card = ReportCard{
//         grade: 1.2f64,
//         student_name: "Hai".to_string(),
//         student_age: 26,
//     };

// }
// pub struct ReportCard <T> {
//     pub grade: T,
//     pub student_name: String,
//     pub student_age: u8,
// }

// impl<T: std::fmt::Debug> ReportCard<T> {
//     pub fn print_grade(&self){
//         // println!("{}", &self.grade);
//         format!("{:#?}", self.grade);
//     }
// }

// Bài 4
// #[derive(Debug)]

// enum Message {
//     // TODO: define a few types of messages as used below
//     Quit,
//     Echo,
//     Move,
//     ChangeColor,
// }
// fn main() {
//     println!("{:?}", Message::Quit);
//     println!("{:?}", Message::Echo);
//     println!("{:?}", Message::Move);
//     println!("{:?}", Message::ChangeColor);
// }

// Bài5
// #[derive(Debug)]
// enum Message {
//     // TODO: define the different variants
//     Move{x:u32, y:u32},
//     Echo(String),
//     ChangeColor(u32, u32, u32),
//     Quit
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}", &self);
//     }
// }
// fn main() {
//     let messages = [
//         Message::Move { x: 10, y: 30 },
//         Message::Echo(String::from("hello world")),
//         Message::ChangeColor(200, 255, 255),
//         Message::Quit,
//     ];

//     for message in &messages {
//         message.call();
//     }
// }

// bài 6, using option, none
// fn print_number(maybe_number: Option<u16>) {
//     println!("printing: {}", maybe_number.unwrap());
// }

// fn main() {
//     print_number(Some(13));
//     print_number(Some(99));
// //  cần khởi tạo
//     let mut numbers: [Option<u16>; 5] = [None,None,None,None,None];
//     for iter in 0..5 {
//         let number_to_add: u16 = {
//             ((iter * 1235) + 2) / (4 * 16)
//         };

//         numbers[iter as usize] = Some(number_to_add);
//     }
// }
// b7 ownership and borrowing
// #[derive(Debug, Clone)]
// struct MyData {
//     val1: i32,
//     val2: String,
// }

// impl MyData {
//     pub fn get_val1(&self) -> i32 {
//         return self.val1.clone();
//     }

//     pub fn get_val2(&self) -> String {
//         return self.val2.clone();
//     }

//     pub fn get_both(&self) -> (i32, String) {
//         return (self.val1, (*self.val2).to_string());
//     }
// }

// fn main() {
//     let d = MyData {
//         val1: 35,
//         val2: String::from("Hello World"),
//     };

//     let both = d.get_both();
//     let x = d.get_val1();
//     let y = d.get_val2();
// }

// bài 8
fn main() {
    let a = A {p: Some("p".to_string())};
    a.a();
}

struct A {
    p: Option<String>
}

impl A {
    fn a(self) -> Self {
        Self::b(&self.p.as_ref().unwrap());
        self
    }
    fn b(b: &str) {
        print!("b: {}", b)
    }
}

