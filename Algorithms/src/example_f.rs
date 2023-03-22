#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Person{
    age: u32,
    name: String,
}

impl Person {
    pub fn new(name: String, age: u32) -> Person{
        Person{
            age,
            name
        }
    }
}

pub fn test(){
    let mut v = vec![
        Person::new("张三".to_string(), 21),
        Person::new("老大".to_string(), 30),
        Person::new("李四".to_string(), 25),
    ];

    v.sort();
    println!("person: {:#?}", v);
    
    v.sort_by(|a, b|{
        // a.age.cmp(&b.age)
        b.age.cmp(&a.age)
    });
    println!("person: {:#?}", v);
}