use anyhow::Result;

#[cfg(test)]
mod tests;

pub fn part_one(report: Vec<&str>) -> Result<u32> {
    let (gamma, epsilon) = compute_rates(report)?;
    let gamma_value = u32::from_str_radix(gamma.as_str(), 2)?;
    let epsilon_value = u32::from_str_radix(epsilon.as_str(), 2)?;
    Ok(gamma_value * epsilon_value)
}

pub fn compute_rates(report: Vec<&str>) -> Result<(String, String)> {
    let entry_length = report[0].len();

    let (gamma, epsilon) = (0..entry_length)
        .map(|i| determine_common_bit_for_column(&report, i).unwrap())
        .fold(
            (String::new(), String::new()),
            |(gamma, epsilon), x| match x {
                '1' => (format!("{}{}", gamma, '1'), format!("{}{}", epsilon, '0')),
                '0' => (format!("{}{}", gamma, '0'), format!("{}{}", epsilon, '1')),
                _ => panic!("foo"),
            },
        );

    Ok((gamma, epsilon))
}

pub fn determine_common_bit_for_column(report: &[&str], column_index: usize) -> Result<char> {
    let (ones, zeros): (Vec<char>, Vec<char>) = report
        .iter()
        .map(|x| (*x).chars().nth(column_index).unwrap())
        .partition(|x| *x == '1');

    return match ones.len() < zeros.len() {
        true => Ok('0'),
        false => Ok('1'),
    };
}

pub fn part_two(report: Vec<&str>) -> Result<u32> {
    let (oxygen, co2) = compute_rates_2(report)?;
    let oxygen_value = u32::from_str_radix(oxygen.as_str(), 2)?;
    let co2_value = u32::from_str_radix(co2.as_str(), 2)?;
    Ok(oxygen_value * co2_value)
}

pub fn compute_rates_2(report: Vec<&str>) -> Result<(String, String)> {
    let oxygen = filter_ratings(report.clone(), 0, |x, y| x.eq(&y))?;
    let co2 = filter_ratings(report.clone(), 0, |x, y| x.ne(&y))?;

    Ok((oxygen, co2))
}

fn filter_ratings(
    values: Vec<&str>,
    index: usize,
    filter_condition: fn(char, char) -> bool,
) -> Result<String> {
    if values.len() == 1 {
        return Ok(values[0].to_string());
    }

    let bit = determine_common_bit_for_column(&values, index)?;

    filter_ratings(
        values
            .into_iter()
            .filter(|x| filter_condition(x.chars().nth(index).unwrap(), bit))
            .collect::<Vec<&str>>(),
        index + 1,
        filter_condition,
    )
}
