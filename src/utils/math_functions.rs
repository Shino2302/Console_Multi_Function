pub fn sum(number_1:f64, number_2:f64) -> f64{
    return number_1 + number_2;
}

pub fn rest(number_1:f64, number_2:f64) -> f64 {
    return number_1 - number_2;
}

pub fn divided(number_1:f64, number_2:f64) -> f64{
    if number_2 == 0.0
    {
        return 0.0;
    }
    else
    {
        return number_1 / number_2;
    }
}

pub fn multiplied(number_1:f64, number_2:f64) -> f64{
    if number_1 == 0.0 || number_2 == 0.0{
        return 0.0;
    }
    else
    {
        return number_1 * number_2;
    }
}