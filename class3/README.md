# Rust入门-第三课
>请基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作。
``````
struct Student {
    id: u32,
    name: String,
    age: u8,
    class: String,
}

struct Club {
    id: u32,
    name: String,
    members: Vec<u32>,
}

struct Course {
    id: u32,
    name: String,
    students: Vec<u32>,
}

struct School {
    students: Vec<Student>,
    clubs: Vec<Club>,
    courses: Vec<Course>,
}

impl School {
    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    fn remove_student(&mut self, student_id: u32) {
        self.students.retain(|s| s.id != student_id);
    }

    fn add_club(&mut self, club: Club) {
        self.clubs.push(club);
    }

    fn remove_club(&mut self, club_id: u32) {
        self.clubs.retain(|c| c.id != club_id);
    }

    fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }

    fn remove_course(&mut self, course_id: u32) {
        self.courses.retain(|c| c.id != course_id);
    }
}

fn main() {
    let mut school = School {
        students: Vec::new(),
        clubs: Vec::new(),
        courses: Vec::new(),
    };

    let student1 = Student {
        id: 1,
        name: String::from("Alice"),
        age: 18,
        class: String::from("Class A"),
    };

    let student2 = Student {
        id: 2,
        name: String::from("Bob"),
        age: 19,
        class: String::from("Class B"),
    };

    let club1 = Club {
        id: 1,
        name: String::from("Chess Club"),
        members: vec![1, 2],
    };

    let course1 = Course {
        id: 1,
        name: String::from("Math"),
        students: vec![1],
    };

    school.add_student(student1);
    school.add_student(student2);
    school.add_club(club1);
    school.add_course(course1);

    println!("Students: {:?}", school.students);
    println!("Clubs: {:?}", school.clubs);
    println!("Courses: {:?}", school.courses);

    school.remove_student(1);
    school.remove_club(1);
    school.remove_course(1);

    println!("Students: {:?}", school.students);
    println!("Clubs: {:?}", school.clubs);
    println!("Courses: {:?}", school.courses);
}


``````
这个示例代码定义了学生（Student）、社团（Club）、课程（Course）和学校（School）这几个基本数据结构。学校结构体包含了学生、社团和课程的集合，并提供了添加和删除这些数据的方法。

在示例的 main 函数中，我们创建了两个学生、一个社团和一个课程，并将它们添加到学校中。然后打印出学校中的学生、社团和课程信息。接着，我们从学校中移除了一个学生、一个社团和一个课程，并再次打印出学校中的信息。

请注意，这只是一个简单的示例，实际的学生管理系统可能需要更复杂的功能和数据结构。希望这个示例能帮助你入门！如果你有任何问题，请随时问我。