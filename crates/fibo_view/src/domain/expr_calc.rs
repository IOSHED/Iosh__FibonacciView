use kalkulator::Expression;

pub async fn calculate_expr(expr: &str) -> Result<u128, String> {
    let mut expr = Expression::new(expr);

    expr.infix_to_postfix().map_err(|e| e.to_string())?;
    expr.compute_expression().map_err(|e| e.to_string())?;
    let res = expr.result.map_err(|e| e.to_string())?;

    Ok(res.floor() as u128)
}
