use errors::Result;

pub trait Operation {
    fn run(&self) -> Result<()>;
}
