pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

impl List {
    pub fn new(items: Vec<i32>) -> Self {
        let mut head = List::Empty;

        for i in items.iter().rev() {
            head = List::Elem(*i, Box::new(head));
        }

        head
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Empty => write!(f, "[END]"),
            List::Elem(i, list) => {
                let _ignored = write!(f, "{i} -> ");
                write!(f, "{list}")
            }
        }
    }
}
