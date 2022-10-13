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
    test_1: ("3 + 4 * 2 / (1 - 5) * 2", -1.0),
    test_2: ("1+1", 2.0),
    test_3: ("-1+1", 0.0),
    test_4: ("-1*2", -2.0),
    test_sin: ("1-sin(90)", 0.0),
    test_cos: ("cos(90)+1", 1.0),
    test_tan: ("tan(0)", 0.0),
}
