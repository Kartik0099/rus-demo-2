use std::fs;

#[allow(dead_code)]
struct Student {
    name: String,
    grade: f64,
}

fn main() {
    let _lines: String = fs::read_to_string("literal-demo").expect("file not found");

    let _students = _lines
        .lines()
        .filter_map(|x| {
            // let (a, b) = x.split_once(' ').unwrap_or(("", ""));
            // println!("a : {} b :{}", a, b);

            let mut s = x.split(' ');
            let name = s.next()?.to_owned();
            // // let b: String = s;
            println!("name : {name} grade {}", s.next().unwrap());

            // println!("remain {}", s.next().unwrap_or("#"));

            //Output from file ->literal-demo
            // name : "kartik grade 4.5",
            // remain #
            // name : "jay grade 2.5",
            // remain #
            // name : "krishna grade 3.5",
            // remain #
            //End

            // let grade = s.next()?.parse::<f64>().ok()?;
            // println!("name :{} grade: {} ", name, grade);
            return Some(Student {
                name: "rahul".to_owned(),
                //  name.to_owned(),
                grade: 3.5,
            });
        })
        .collect::<Vec<_>>();
}
