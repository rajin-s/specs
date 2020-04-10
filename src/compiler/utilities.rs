pub struct TempName
{
    name:   String,
    number: usize,
}
impl TempName
{
    pub fn new(name: &str) -> Self
    {
        return Self {
            name:   name.to_owned(),
            number: 0,
        };
    }

    pub fn next(&mut self) -> String
    {
        self.number += 1;
        format!("_{}_{}", self.name, self.number)
    }
}