fn main() {
    let input = include_str!("1_input.txt");

    let modules_mass: Vec<isize> = input.split('\n')
        .map(|mass| mass.parse::<isize>().expect("we should be able to parse this"))
        .collect();

    let fuel_requirements: isize = modules_mass.iter()
        .map(|mass| mass / 3 - 2)
        .sum();

    println!("Part 1: {}", fuel_requirements);

    let fuel_requirements: isize = modules_mass.iter()
        .map(|mass| {
            let mut result = 0;
            let mut last_fuel = mass / 3 - 2;
            while last_fuel > 0 {
                result += last_fuel;
                last_fuel = last_fuel / 3 - 2;
            }

            result
        })
        .sum();

    println!("Part 2: {}", fuel_requirements);
}