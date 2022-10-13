use calc::CalcConf;

mod calc;

/// Creates new calculator.
/// Its thread safe and can be reused multiptle times.
pub fn new() -> impl Calc {
    calc::SimpleCalc { conf: CalcConf::new() }
}

/// Basic calc trait.
pub trait Calc {
    fn calc(&self, input: &str) -> Result<f64, CalcErr>;
}

/// Calculation error type.
#[derive(Debug)]
pub struct CalcErr(pub String);
