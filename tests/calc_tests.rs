use rust_calc;
use rust_calc::Calc;

macro_rules! calc_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            let calc = rust_calc::new();
            let act = calc.calc(input).unwrap();
            assert_eq!(expected, act);
        }
    )*
    }
}

calc_tests! {
    simple_add: ("2+2", 4.0),
    simple_substract: ("10-1", 9.0),
    complex: ("3 + 4 * 2 / (1 - 5) * 2", -1.0),
    add_first_neg: ("-1+1", 0.0),
    mult_first_neg: ("-1*2", -2.0),
    simple_sin: ("1-sin(90)", 0.0),
    simple_cos: ("cos(90)+1", 1.0),
    simple_tan: ("tan(0)", 0.0),
    incomplete_decimals: ("10.0+10.", 20.0),
    simpl_exp: ("2^10", 1024.0),
}
