use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("welcome {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("welcome {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("probatory member {}", self.name),
            VisitorAction::Refuse => println!("{} not allowed", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to real line");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", "hello Bert", VisitorAction::Accept, 45),
        Visitor::new(
            "steve",
            "hi steve",
            VisitorAction::AcceptWithNote {
                note: String::from("Crazy Folk"),
            },
            15,
        ),
        Visitor::new("fred", "broo", VisitorAction::Refuse, 30),
    ];

    loop {
        println!("Hello, whats your name?");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} not on the list", name);
                    visitor_list.push(Visitor::new(
                        &name,
                        "New Friend",
                        VisitorAction::Probation,
                        0,
                    ));
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
