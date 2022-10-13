use calc::Calc;

pub mod calc;

pub fn simple_calc() -> impl calc::Calc {
    calc::SimpleCalc::new()
}
