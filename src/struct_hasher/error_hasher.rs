use std;
use std::fmt;

const MESSAGE: &str = "Не указаны необходимые значения для запуска приложения";
#[derive(Debug, Clone)]
pub struct WrongArgs{
    pub messge: String
}

impl fmt::Display for WrongArgs{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>{
        write!(f, "{}", self.messge)
    }
}

impl std::error::Error for WrongArgs{
    fn description(&self) -> &str{
        &self.messge
    }
}

impl WrongArgs{
    pub fn of() -> WrongArgs{
        WrongArgs{messge:MESSAGE.to_string()}
    }
}