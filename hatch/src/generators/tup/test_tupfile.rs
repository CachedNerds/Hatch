pub struct Tupfile;

impl Tupfile {
    pub fn new() -> Tupfile {
        Tupfile
    }

    pub fn name() -> String {
        String::from("Tupfile")
    }
}

impl ToString for Tupfile {
    fn to_string(&self) -> String {
        String::from(".gitignore")
    }
}
