pub trait Cutter {
    fn cut_off(&self) -> String;
}
impl Cutter for String {
    fn cut_off(&self) -> String {
        self.trim()
            .replace("\n", "")
            .replace("\t", "")
            .replace("\"", "")
    }
}
impl Cutter for &str {
    fn cut_off(&self) -> String {
        self.trim()
            .replace("\n", "")
            .replace("\t", "")
            .replace("\"", "")
    }
}
