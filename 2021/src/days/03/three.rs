use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

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
        .map(|i| determine_common_bit_for_column(&report, i, '1').unwrap())
        .map(|bit| match bit {
            '1' => (1, 0),
            '0' => (0, 1),
            _ => panic!("foooo"),
        })
        .fold(
            (String::new(), String::new()),
            |(gamma, epsilon), (x, y)| (format!("{}{}", gamma, x), format!("{}{}", epsilon, y)),
        );

    Ok((gamma, epsilon))
}

pub fn determine_common_bit_for_column(
    report: &[&str],
    column_index: usize,
    tie: char,
) -> Result<char> {
    let (ones, zeros): (Vec<char>, Vec<char>) = report
        .iter()
        .map(|x| (*x).chars().nth(column_index).unwrap())
        .partition(|x| *x == '1');

    println!("{} vs {}", ones.len(), zeros.len());
    return if ones.len() > zeros.len() {
        Ok('1')
    } else if ones.len() < zeros.len() {
        Ok('0')
    } else {
        println!("picking {}", tie);
        Ok(tie)
    };
}

pub fn part_two(report: Vec<&str>) -> Result<u32> {
    let (gamma, epsilon) = compute_rates_2(report)?;
    let gamma_value = u32::from_str_radix(gamma.as_str(), 2)?;
    let epsilon_value = u32::from_str_radix(epsilon.as_str(), 2)?;
    Ok(gamma_value * epsilon_value)
}

pub fn compute_rates_2(report: Vec<&str>) -> Result<(String, String)> {
    let oxygen = filter_ratings(report.clone(), 0, |x, y| x.eq(&y), '1')?;
    let co2 = filter_ratings(report.clone(), 0, |x, y| x.ne(&y), '1')?;

    Ok((oxygen, co2))
}

fn filter_ratings(
    values: Vec<&str>,
    index: usize,
    filter_condition: fn(char, char) -> bool,
    tie: char,
) -> Result<String> {
    if values.len() == 1 {
        return Ok(values[0].to_string());
    }

    println!("{:?}", &values);
    let bit = determine_common_bit_for_column(&values, index, tie)?;

    filter_ratings(
        values
            .into_iter()
            .filter(|x| filter_condition(x.chars().nth(index).unwrap(), bit))
            .collect::<Vec<&str>>(),
        index + 1,
        filter_condition,
        tie,
    )
}
