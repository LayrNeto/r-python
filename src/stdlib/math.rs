use crate::ir::ast::{EnvValue, Expression};
use crate::ir::ast::Environment; 
use std::collections::HashMap;

pub fn sqrt(args: Vec<EnvValue>) -> Result<EnvValue, String> {
    if args.len() != 1 {
        return Err("sqrt expects exactly one argument".to_string());
    }

    if let EnvValue::Exp(Expression::CReal(x)) = &args[0] {
        Ok(EnvValue::Exp(Expression::CReal(x.sqrt())))
    } else {
        Err("sqrt expects a real number argument".to_string())
    }
}

pub fn factorial(args: Vec<EnvValue>) -> Result<EnvValue, String> {
    if args.len() != 1 {
        return Err("factorial expects exactly one argument".to_string());
    }

    if let EnvValue::Exp(Expression::CInt(n)) = &args[0] {
        if *n < 0 {
            return Err("factorial expects a non-negative integer argument".to_string());
        }
        let mut prod: i32 = 1;
        for i in 1..=*n {
            prod *= i;
        }
        Ok(EnvValue::Exp(Expression::CInt(prod)))
    } else {
        Err("factorial expects an integer argument".to_string())
    }
}

pub fn gcd(args: Vec<EnvValue>) -> Result<EnvValue, String> {
    if args.len() != 2 {
        return Err("gcd expects exactly two arguments".to_string());
    }

    if let (EnvValue::Exp(Expression::CInt(a)), EnvValue::Exp(Expression::CInt(b))) =
        (&args[0], &args[1])
    {
        let mut a = *a;
        let mut b = *b;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        Ok(EnvValue::Exp(Expression::CInt(a.abs())))
    } else {
        Err("gcd expects two integer arguments".to_string())
    }
}


pub fn lcm(args: Vec<EnvValue>) -> Result<EnvValue, String> {
    if args.len() != 2 {
        return Err("lcm expects exactly two arguments".to_string());
    }

    if let (EnvValue::Exp(Expression::CInt(a)), EnvValue::Exp(Expression::CInt(b))) =
        (&args[0], &args[1])
    {
        let gcd_val = match gcd(args.clone()) {
            Ok(EnvValue::Exp(Expression::CInt(val))) => val,
            Err(err) => return Err(format!("Error calculating gcd: {}", err)),
            _ => return Err("Unexpected error in gcd calculation".to_string()),
        };

        let lcm_val = (a * b).abs() / gcd_val;
        Ok(EnvValue::Exp(Expression::CInt(lcm_val)))
    } else {
        Err("lcm expects two integer arguments".to_string())
    }
}

pub fn comb(args: Vec<EnvValue>) -> Result<EnvValue, String> {
    if args.len() != 2 {
        return Err("comb expects exactly two arguments".to_string());
    }

    if let (EnvValue::Exp(Expression::CInt(n)), EnvValue::Exp(Expression::CInt(k))) = (&args[0], &args[1]) {
        if *n < 0 || *k < 0 {
            return Err("comb expects non-negative integers".to_string());
        }
        let n = *n;
        let mut k = *k;
        if k > n {
            return Ok(EnvValue::Exp(Expression::CInt(0)));
        }
        if k > n - k {
            k = n - k;
        }
        let result = (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1));
        Ok(EnvValue::Exp(Expression::CInt(result)))
    } else {
        Err("comb expects two integer arguments".to_string())
    }
}

pub fn perm(args: Vec<EnvValue>) -> Result<EnvValue, String> {
    if args.len() != 2 {
        return Err("perm expects exactly two arguments".to_string());
    }

    if let (EnvValue::Exp(Expression::CInt(n)), EnvValue::Exp(Expression::CInt(k))) = (&args[0], &args[1]) {
        if *n < 0 || *k < 0 {
            return Err("perm expects non-negative integers".to_string());
        }
        let n = *n;
        let k = *k;
        if k > n {
            return Ok(EnvValue::Exp(Expression::CInt(0)));
        }
        let mut result: i32 = 1;
        for i in 0..k {
            result *= n - i;
        }
        Ok(EnvValue::Exp(Expression::CInt(result)))
    } else {
        Err("perm expects two integer arguments".to_string())
    }
}

//AINDA EH NECESSÁRIO APLICAR CONVERSAO
pub fn euclidean_distance(p: &[f64], q: &[f64]) -> Result<f64, String> {
    if p.len() != q.len() {
        return Err("Os pontos devem ter a mesma dimensão.".to_string());
    }

    let mut sum_of_squares = 0.0;
    for i in 0..p.len() {
        let diff = p[i] - q[i];
        sum_of_squares += diff * diff;
    }

    Ok(sum_of_squares.sqrt())
}

pub fn is_prime(n: u64) -> bool {
   //Casos base: 
   //1 - Se n <= 1, não eh primo
   //2 - 2 e 3 são primos
   //3 - se n eh divisivel por 2 ou 3, nao eh primo
   
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

pub fn log(base: f64, x: f64) -> f64 {
    x.log(base)
}

pub fn sum_of_products(p: &[f64], q: &[f64]) -> Result<f64, String> {
    if p.len() != q.len() {
        return Err("Os iteráveis devem ter o mesmo comprimento.".to_string());
    }

    let mut sum = 0.0;
    for i in 0..p.len() {
        sum += p[i] * q[i];
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

//TESTES FUNCAO SQRT
    #[test]
    fn test_sqrt_positive_real() {
        let result = sqrt(vec![EnvValue::Exp(Expression::CReal(9.0))]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CReal(res_value))) = result {
            assert_eq!(res_value, 3.0);
        }

        let result = sqrt(vec![EnvValue::Exp(Expression::CReal(49.0))]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CReal(res_value))) = result {
            assert_eq!(res_value, 7.0);
        }

        let result = sqrt(vec![EnvValue::Exp(Expression::CReal(121.0))]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CReal(res_value))) = result {
            assert_eq!(res_value, 11.0);
        }
    }

    #[test]
    fn test_sqrt_zero() {
        let result = sqrt(vec![EnvValue::Exp(Expression::CReal(0.0))]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CReal(res_value))) = result {
            assert_eq!(res_value, 0.0);
        }
    }

    #[test]
    fn test_sqrt_invalid_number_of_arguments() {
        let result = sqrt(vec![]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "sqrt expects exactly one argument");
    }

    #[test]
    fn test_sqrt_invalid_number_of_arguments_multiple() {
        let result = sqrt(vec![
            EnvValue::Exp(Expression::CReal(25.0)),
            EnvValue::Exp(Expression::CReal(9.0)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "sqrt expects exactly one argument");
    }

    #[test]
    fn test_sqrt_invalid_argument_type() {
        let result = sqrt(vec![EnvValue::Exp(Expression::CInt(25))]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "sqrt expects a real number argument");
    }
//TESTES FUNCAO FACTORIAL
    #[test]
    fn test_factorial_valid_inputs() {
        let result = factorial(vec![EnvValue::Exp(Expression::CInt(0))]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 1);
        }

        let result = factorial(vec![EnvValue::Exp(Expression::CInt(1))]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 1);
        }

        let result = factorial(vec![EnvValue::Exp(Expression::CInt(5))]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 120);
        }

        let result = factorial(vec![EnvValue::Exp(Expression::CInt(10))]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 3628800);
        }
    }

    #[test]
    fn test_factorial_invalid_number_of_arguments() {
        let result = factorial(vec![]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "factorial expects exactly one argument");
    }

    #[test]
    fn test_factorial_invalid_number_of_arguments_multiple() {
        let result = factorial(vec![
            EnvValue::Exp(Expression::CInt(1)),
            EnvValue::Exp(Expression::CInt(2)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "factorial expects exactly one argument");
    }

    #[test]
    fn test_factorial_invalid_argument_type() {
        let result = factorial(vec![EnvValue::Exp(Expression::CReal(3.5))]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "factorial expects an integer argument");
    }

    #[test]
    fn test_factorial_negative_argument() {
        let result = factorial(vec![EnvValue::Exp(Expression::CInt(-1))]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "factorial expects a non-negative integer argument");
    }
//TESTES FUNCAO GCD
    #[test]
    fn test_gcd_valid_inputs() {
        let result = gcd(vec![
            EnvValue::Exp(Expression::CInt(48)),
            EnvValue::Exp(Expression::CInt(18)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 6);
        }

        let result = gcd(vec![
            EnvValue::Exp(Expression::CInt(7)),
            EnvValue::Exp(Expression::CInt(3)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 1);
        }

        let result = gcd(vec![
            EnvValue::Exp(Expression::CInt(-48)),
            EnvValue::Exp(Expression::CInt(18)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 6);
        }

        let result = gcd(vec![
            EnvValue::Exp(Expression::CInt(0)),
            EnvValue::Exp(Expression::CInt(18)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 18);
        }
    }

    #[test]
    fn test_gcd_invalid_number_of_arguments() {
        let result = gcd(vec![EnvValue::Exp(Expression::CInt(48))]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "gcd expects exactly two arguments");
    }

    #[test]
    fn test_gcd_invalid_number_of_arguments_multiple() {
        let result = gcd(vec![
            EnvValue::Exp(Expression::CInt(48)),
            EnvValue::Exp(Expression::CInt(18)),
            EnvValue::Exp(Expression::CInt(6)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "gcd expects exactly two arguments");
    }

    #[test]
    fn test_gcd_invalid_argument_type() {
        let result = gcd(vec![
            EnvValue::Exp(Expression::CReal(48.0)),
            EnvValue::Exp(Expression::CInt(18)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "gcd expects two integer arguments");
    }
//TESTES PARA LCM
    #[test]
    fn test_lcm_valid_inputs() {
        let result = lcm(vec![
            EnvValue::Exp(Expression::CInt(48)),
            EnvValue::Exp(Expression::CInt(18)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 144);
        }

        let result = lcm(vec![
            EnvValue::Exp(Expression::CInt(7)),
            EnvValue::Exp(Expression::CInt(3)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 21);
        }

        let result = lcm(vec![
            EnvValue::Exp(Expression::CInt(-48)),
            EnvValue::Exp(Expression::CInt(18)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 144);
        }

        let result = lcm(vec![
            EnvValue::Exp(Expression::CInt(0)),
            EnvValue::Exp(Expression::CInt(18)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 0);
        }
    }

    #[test]
    fn test_lcm_invalid_number_of_arguments() {
        let result = lcm(vec![EnvValue::Exp(Expression::CInt(48))]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "lcm expects exactly two arguments");
    }

    #[test]
    fn test_lcm_invalid_number_of_arguments_multiple() {
        let result = lcm(vec![
            EnvValue::Exp(Expression::CInt(48)),
            EnvValue::Exp(Expression::CInt(18)),
            EnvValue::Exp(Expression::CInt(6)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "lcm expects exactly two arguments");
    }

    #[test]
    fn test_lcm_invalid_argument_type() {
        let result = lcm(vec![
            EnvValue::Exp(Expression::CReal(48.0)),
            EnvValue::Exp(Expression::CInt(18)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "lcm expects two integer arguments");
    }

//TESTES PARA COMB
    #[test]
    fn test_comb_valid_inputs() {
        let result = comb(vec![
            EnvValue::Exp(Expression::CInt(5)),
            EnvValue::Exp(Expression::CInt(2)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 10);
        }

        let result = comb(vec![
            EnvValue::Exp(Expression::CInt(10)),
            EnvValue::Exp(Expression::CInt(3)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 120);
        }

        let result = comb(vec![
            EnvValue::Exp(Expression::CInt(5)),
            EnvValue::Exp(Expression::CInt(6)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 0);
        }
    }

    #[test]
    fn test_comb_invalid_number_of_arguments() {
        let result = comb(vec![EnvValue::Exp(Expression::CInt(5))]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "comb expects exactly two arguments");
    }

    #[test]
    fn test_comb_invalid_number_of_arguments_multiple() {
        let result = comb(vec![
            EnvValue::Exp(Expression::CInt(5)),
            EnvValue::Exp(Expression::CInt(2)),
            EnvValue::Exp(Expression::CInt(1)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "comb expects exactly two arguments");
    }

    #[test]
    fn test_comb_invalid_argument_type() {
        let result = comb(vec![
            EnvValue::Exp(Expression::CReal(5.0)),
            EnvValue::Exp(Expression::CInt(2)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "comb expects two integer arguments");
    }

    #[test]
    fn test_comb_negative_arguments() {
        let result = comb(vec![
            EnvValue::Exp(Expression::CInt(5)),
            EnvValue::Exp(Expression::CInt(-2)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "comb expects non-negative integers");
    }

//TESTES PARA PERM
    #[test]
    fn test_perm_valid_inputs() {
        let result = perm(vec![
            EnvValue::Exp(Expression::CInt(5)),
            EnvValue::Exp(Expression::CInt(2)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 20);
        }

        let result = perm(vec![
            EnvValue::Exp(Expression::CInt(10)),
            EnvValue::Exp(Expression::CInt(3)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 720);
        }

        let result = perm(vec![
            EnvValue::Exp(Expression::CInt(5)),
            EnvValue::Exp(Expression::CInt(6)),
        ]);
        assert!(result.is_ok());
        if let Ok(EnvValue::Exp(Expression::CInt(value))) = result {
            assert_eq!(value, 0);
        }
    }

    #[test]
    fn test_perm_invalid_number_of_arguments() {
        let result = perm(vec![EnvValue::Exp(Expression::CInt(5))]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "perm expects exactly two arguments");
    }

    #[test]
    fn test_perm_invalid_number_of_arguments_multiple() {
        let result = perm(vec![
            EnvValue::Exp(Expression::CInt(5)),
            EnvValue::Exp(Expression::CInt(2)),
            EnvValue::Exp(Expression::CInt(1)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "perm expects exactly two arguments");
    }

    #[test]
    fn test_perm_invalid_argument_type() {
        let result = perm(vec![
            EnvValue::Exp(Expression::CReal(5.0)),
            EnvValue::Exp(Expression::CInt(2)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "perm expects two integer arguments");
    }

    #[test]
    fn test_perm_negative_arguments() {
        let result = perm(vec![
            EnvValue::Exp(Expression::CInt(5)),
            EnvValue::Exp(Expression::CInt(-2)),
        ]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "perm expects non-negative integers");
    }

//AINDA EH NECESSARIO ADAPTAR
//TESTES FUNCAO EUCLIDES
    #[test]
    fn test_euclidean_distance() {
        let p = vec![1.0, 2.0, 3.0];
        let q = vec![4.0, 5.0, 6.0];
        let result = euclidean_distance(&p, &q);
        assert!(result.is_ok()); 
        assert!((result.unwrap() - 5.196152422706632).abs() < 1e-10);

        let p = vec![0.0, 0.0];
        let q = vec![3.0, 4.0];
        let result = euclidean_distance(&p, &q);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
}
    #[test]
    fn test_euclidean_distance_different_dimensions() {
        let p = vec![1.0, 2.0];
        let q = vec![1.0, 2.0, 3.0];
        let result = euclidean_distance(&p, &q);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Os pontos devem ter a mesma dimensão.");
}
//TESTES PARA FUNCAO IS PRIME
    #[test]
    fn test_is_prime_small_numbers() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6)); 
        assert!(is_prime(7));  
        assert!(!is_prime(8)); 
        assert!(!is_prime(9)); 
        assert!(!is_prime(10)); 

    }

    #[test]
    fn test_is_prime_medium_numbers() {
        assert!(!is_prime(15));
        assert!(!is_prime(25));
        assert!(is_prime(29));
        assert!(!is_prime(49));
        assert!(is_prime(53));
    }
//TESTES PARA FUNCAO LOG
    #[test]
    fn test_log_valid_inputs() {
        assert!((log(2.0, 8.0) - 3.0).abs() < 1e-10);
        assert!((log(10.0, 100.0) - 2.0).abs() < 1e-10);
        assert!((log(3.0, 9.0) - 2.0).abs() < 1e-10);
        assert!((log(5.0, 1.0) - 0.0).abs() < 1e-10); 
    }

    #[test]
    fn test_log_invalid_inputs() {
        assert!(log(2.0, 0.0).is_nan());
        assert!(log(2.0, -1.0).is_nan());
        assert!(log(0.0, 10.0).is_nan());
        assert!(log(-1.0, 10.0).is_nan());
        assert!(log(1.0, 10.0).is_nan());
    }
//TESTES PARA FUNCAO SOMA DE PRODUTOS
    #[test]
    fn test_sum_of_products() {
        let p = vec![1.0, 2.0, 3.0];
        let q = vec![4.0, 5.0, 6.0];
        let result = sum_of_products(&p, &q);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 32.0);
    
        let p = vec![0.0, 1.0, 2.0];
        let q = vec![3.0, 4.0, 5.0];
        let result = sum_of_products(&p, &q);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 14.0);
    }

    #[test]
    fn test_sum_of_products_different_lengths() {
        let p = vec![1.0, 2.0];
        let q = vec![1.0, 2.0, 3.0];
        let result = sum_of_products(&p, &q);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Os iteráveis devem ter o mesmo comprimento.");
    }

} 