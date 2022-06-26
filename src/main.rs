use std::collections::HashMap;

#[derive(Debug)]
pub struct School {
    students:HashMap<String, u32>
}

impl School {
    //0. Tạo 1 function new() khởi tạo rỗng ban đầu cho danh sách
    pub fn new() -> School {
        School { students: HashMap::new() }
    }

    //1. Có thể thêm thông tin của sinh viên gồm có tên và điểm
    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(student.to_string(), grade);
    }

    //2. Liệt kê các điểm số hiện tại mà trường đã cập nhập
    pub fn grades(&self) -> Vec<u32> {
        let mut result = Vec::new();
        for (_, value) in self.students.iter() {
            result.push(*value)
        };

        // to filter same value in Vec
        result.sort();
        result.dedup(); 
        result
    }

    //3. Liệt kê danh sách các học sinh có cùng 1 điểm số
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut result = Vec::new();

        for (key, value) in self.students.iter() {
            if grade == *value {
                result.push(key.to_string());
            } else {
                
            }
        };
        result.sort(); // to change to alphabet
        result
    }
}


fn main() {
    println!("Hello, world!");

    let mut school = School::new();
    println!("School rỗng: {:?}", school);

    school.add(7, "Alice");
    school.add(2, "Bob");
    println!("School sau khi thêm: {:?}", school);

    school.add(2, "Steve");
    let grades = school.grades();
    println!("Các số điểm hiện tại của School: {:?}", grades);

    school.add(2, "Alex");
    let grade = school.grade(2);
    println!("Danh sách học sinh có cùng điểm số 2: {:?}", grade);
    
}
