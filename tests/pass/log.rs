use log_attributes::log;

#[log(trace, "called fn {fn}")]
fn empty() {}

#[log(info, "called async fn {fn}")]
async fn empty_async() {}

#[log(info, "called fn {fn}")]
fn simple_body() {
    let n = 40 + 2;
    println!("Hello World! {}", n);
}

#[log(info, "called fn {fn} with {a} and {b}")]
fn with_args(a: u32, b: &str) {
    let _ = (a, b);
}

#[log(info, "called fn {fn} with {a} which resulted in {return}")]
fn with_return(a: u32) -> u32 {
    a
}

#[log(info, "called fn {fn} not with {{a}} but with {b}")]
fn escaped(b: u32) {}

#[derive(Debug)]
struct S(u32);

impl S {
    #[log(info, "called fn {fn} with {self:?} which resulted in {return}")]
    fn with_self(self) -> u32 {
        self.0
    }

    #[log(info, "called fn {fn} with {self:?} which resulted in {return}")]
    fn with_self_ref(&self) -> &u32 {
        &self.0
    }

    #[log(info, "called fn {fn} which resulted in {return}")]
    fn with_self_mut(&mut self) -> &mut u32 {
        &mut self.0
    }

    #[log(info, "called async fn {fn} with {self:?} which resulted in {return}")]
    async fn async_self(self) -> u32 {
        self.0
    }

    #[log(info, "called async fn {fn} with {self:?} which resulted in {return}")]
    async fn async_self_ref(&self) -> &u32 {
        &self.0
    }

    #[log(info, "called async fn {fn} which resulted in {return}")]
    async fn async_self_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}


fn main() {}
