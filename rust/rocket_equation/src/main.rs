use std::fs;

fn compute_fuel(mass: i32) -> i32{
    (mass / 3) - 2
}

fn compute_all_fuel(mass: i32) -> i32 {
    if compute_fuel(mass) < 1{
        return 0;
    }
    else {
        return compute_fuel(mass) + compute_all_fuel(compute_fuel(mass));
    }
}

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(&filename).expect("Couldn't read the file");

    let mut total_launch_fuel = 0;
    let mut total_fuel_needed = 0;

    for input_mass in contents.lines() {
        let input_mass: i32 = input_mass.trim().parse().expect("Not a number");
        total_launch_fuel += compute_fuel(input_mass);
        total_fuel_needed += compute_all_fuel(input_mass);
    }

    println!("Total launch fuel needed: {}", total_launch_fuel);
    println!("Total fuel needed: {}", total_fuel_needed);    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_fuel() {
        assert_eq!(compute_fuel(12), 2);
        assert_eq!(compute_fuel(14), 2);
        assert_eq!(compute_fuel(1969), 654);
        assert_eq!(compute_fuel(100756), 33583);
        assert_eq!(compute_fuel(654), 216);
        assert_eq!(compute_fuel(compute_fuel(654)), 70);
    }

    #[test]
    fn test_compute_all_fuel() {
        assert_eq!(compute_all_fuel(14), 2);
        assert_eq!(compute_all_fuel(21), 5);
        assert_eq!(compute_all_fuel(1969), 966);
        assert_eq!(compute_all_fuel(100756), 50346);
    }
}
