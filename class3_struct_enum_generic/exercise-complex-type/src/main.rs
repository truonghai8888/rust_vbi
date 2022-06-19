use std::collections::HashMap;
// Bài tập
// Cho ngữ cảnh như sau : Một ngôi trường cần lập danh
//sách thông tin sinh viên bao gồm tên sinh viên và điểm của sinh viên đó.
// với mục đích thống kê kiểm tra giáo dục của ngôi trường này

fn main() {
// Các bạn phải vuợt qua một số test case như sau :

/*-----------------------------*/
//Test case 1: Khởi tạo đầu tiên danh sách phải rỗng
    let mut school_students_list = School::new();

/*-----------------------------*/

// Test case 2:
//Thêm sinh viên có tên "Lee" với điểm số là 2
// thì tất cả các điểm số hiện có của trường là 2
//nếu thêm sinh viên khác "Nancy" với điểm số là 3
//thì các điểm số hiện tại là [2,3]
    school_students_list.add(2, "Lee");
    println!("Test case 2.1:{:?}", school_students_list.grades());
    school_students_list.add(3, "Nancy");
    println!("Test case 2.2:{:?}", school_students_list.grades());

/*-----------------------------*/

// Test case 3:
// Giả sử danh sách hiện tại : [(Bob, 4), (Alice,4), (Tom,5)]
// với điểm số 4 thì ta có sinh viên nào: -> [Alice, Bob] not [Bob ,Alice]
// vì cần tên theo alphabet
    school_students_list.add(4, "Bob");
    school_students_list.add(4, "Alice");
    school_students_list.add(5, "Tom");
    println!("Test case 3:{:?}", school_students_list.grade(4));

}


/*-----------------------------*/
// Nếu mọi người làm xong rùi thì có thể làm advance hơn bằng cách 
// sử dụng Generic type cho điểm số không nhất thiết phải U32 nữa mà có thể "A+", "B+" chẳng hạn (string)
/*-----------------------------*/

// Sườn thông tin cho mọi người dễ làm


/*-----------------------------*/
// Gợi ý:
// Định nghĩa bằng struct, mọi người nên sử dụng HashMap 
// Tại sao lại sử dụng HashMap và ko phải Vec
//https://doc.rust-lang.org/std/collections/struct.HashMap.html
// struct School {
//     students: HashMap<String, u32>
// }
// trong trường hợp này thì String : tên của sinh viên đó
// u32 là điểm số 

pub struct School {
    // Dinh nghia theo dang key:value
    students: HashMap<String, u32>
}

// Một số yêu cầu như sau:

impl School {
/*-----------------------------*/
//0. Tạo 1 function new() khởi tạo rỗng ban đầu cho danh sách
    pub fn new() -> School {
        Self {
            students: HashMap::new(),
        }

    }

/*-----------------------------*/
//1. Có thể thêm thông tin của sinh viên gồm có tên và điểm
// Ví dụ: thêm tên "Alice" có 7 điểm, thêm tên "Bob" có 2 điểm, ...
// Gợi ý : định nghĩa hàm "add" implement cho Struct
    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(student.to_string(), grade);
    }

/*-----------------------------*/
//2. Liệt kê các điểm số hiện tại mà trường đã cập nhập
// ví dụ :danh sách hiện tại gồm có [(Alice, 10), (Bob,4)]
//trả về là [4,10] (điểm số nên tăng dần và ko có duplicate)
// ví dụ: [(Alice, 10), (Bob,4), (Steve,4)] -> [4,10]
    pub fn grades(&self) -> Vec<u32> {
        // tạo vec bằng cách lấy ra list values 
        let mut grades: Vec<u32> = self.students.values().cloned().collect();
        // dung de dua cac gia tri giong nhau về các vị trí liên tiếp
        grades.sort();
        // loại bo các giá trị giống nhau
        grades.dedup();
        // returrn
        grades
    }

/*-----------------------------*/
//3. Liệt kê danh sách các học sinh có cùng 1 điểm số
// ví dụ: hiện tại danh sách gồm có (Alice, 3), (Bob, 10), (Charlie,3)
// liệt kê danh sách học sinh có cùng 3 điểm : [Alice, Charlie]

// Yêu cầu trả về: danh sách sinh viên là alphabet theo tên 

// Gợi ý: 
// ví dụ : Ban đầu [(Alice, 10), (Bob,2), (Eve,4), (Long,2)] -> [(Bob, 2), (Long,2), (Eve,4), (Alice,10)]
//định nghĩa hàm "find_student" có tham số là điểm số -> trả về danh sách các sinh viên có cùng điểm số mong muốn
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut names: Vec<String> = self.students.
            iter().filter(|student| *(student.1) == grade)
            .map(|student| student.0.clone())
            .collect();
        names.sort();
        names
    }
}










