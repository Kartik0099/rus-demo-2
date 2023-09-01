use std::{fs, str::FromStr};

struct Student {
    name: String,
    grade: f64,
}

impl FromStr for Student {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, g) = s.split_once(' ').unwrap_or(("", ""));

        let _grade = g.trim().parse::<f64>();
        match _grade {
            Ok(x) => {
                return Ok(Student {
                    name: name.to_owned(),
                    grade: x,
                });
            }
            Err(e) => {
                eprintln!("Error from grade {}", e);
                return Err(());
            }
        }
    }
}

#[allow(dead_code)]
fn print_higher_grade_for_student(s: Vec<&str>) -> () {
    s.iter()
        .map(|line| {
            let mut s = line.split(' ');

            let name = s.next()?.to_owned();
            let grade = s.next()?.parse::<f64>().ok()?;
            // println!("map ->{} : {} ", &name, &grade);
            return Some(Student {
                name: name.to_owned(),
                grade,
            });
        })
        .flatten()
        .filter(|s| s.grade >= 3.5)
        .for_each(|s| {
            println!("{}: {}", s.name, s.grade);
        })
}

#[allow(dead_code)]
fn main_2() -> () {
    println!("Here run");

    let _line = fs::read_to_string("student-grades").expect("file not found");

    let students: Vec<Student> = _line
        .lines()
        .filter_map(|x| {
            return x.parse::<Student>().ok();
        })
        .collect::<Vec<_>>();

    println!("Parse Student logic :start");
    students.iter().filter(|s| s.grade >= 3.5).for_each(|s| {
        println!("{}: {}", s.name, s.grade);
    });

    println!("Parse Student logic :end \n \n \n");

    // let b = "3.5";

    // let c = b.parse::<f64>().expect("float value in string");

    // println!("{}", c);

    println!("Line :start");
    let s1: Vec<&str> = _line.lines().collect::<Vec<&str>>();
    print_higher_grade_for_student(s1);
    println!("Line :End\n\n");

    let stud: [&str; 5] = [
        "kartik 4.5",
        "jay 2.5",
        "krishna 3.5",
        "mahesh 2.9",
        "vijay 3.8",
    ];

    print_higher_grade_for_student(stud.to_vec());
}

fn main() {}
