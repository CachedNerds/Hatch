pub struct TupfileIni;

impl TupfileIni {
    pub fn new() -> TupfileIni {
        TupfileIni
    }

    pub fn name() -> String {
        String::from("Tupfile.ini")
    }
}

impl ToString for TupfileIni {
    fn to_string(&self) -> String {
        String::from("")
    }
}
