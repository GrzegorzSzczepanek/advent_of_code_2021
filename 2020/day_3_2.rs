use std::fs::read_to_string;

fn check(passport: String, fields: &Vec<&str>) -> bool {
    
    for field in fields {
        // let mut s = String::from(field.to_string());
        if !passport.contains(field) {
            return false;
        }
        // passport = passport.replace("\n", " ").replace(format!("{}:", field).as_str(), "");
        // if passport.contains("cid") {
        //     passport = passport.replace("cid:", "");
        
        
    }
    // passport = passport
    let passport_fields: Vec<String> = passport
        .replace("\n", " ")
        .split(" ").map(|s| s.to_string())
        .collect();
    
    for field in passport_fields {
        let field1 = field.clone();
        if field.contains("byr") {
            let field1: i32 = field.replace("byr:", "").parse().unwrap();
            if field1 < 1920 || field1 > 2002 {

                            return false;
            }
        }
        if field.contains("iyr") {
            let field1: i32 = field.replace("iyr:", "").parse().unwrap();
            if field1 < 2010 || field1 > 2020 {

                return false;
            }
        }
        if field.contains("eyr") {
            let field1: i32 = field.replace("eyr:", "").parse().unwrap();
            if field1 < 2020 || field1 > 2030 {

                return false;
            }
        }
        if field.contains("hgt") {
            if field1.contains("cm") {
                let field1: i32 = field.replace("hgt:", "").replace("cm", "").parse().unwrap();

                if !(150..193).contains(&field1) {
                    return false;
                }
            }
            else if field1.contains("in") {
                let field1: i32 = field.replace("hgt:", "").replace("in", "").parse().unwrap();

                if !(59..76).contains(&field1) {
                    return false;
                }
            }
        }
        if field.contains("hcl") {


            let field1: String = field.replace("hcl:#", "");
            if !field1.chars().all(|c| c.is_alphanumeric()) {

                return false;
            };

        }
            
        if field.contains("ecl") {
            let field1: String = field.replace("ecl:", "");

            if !"amb blu brn gry grn hzl oth".contains(field1.as_str()) {

                return false;
            }
        }
        if field.contains("pid") {
            let field1: String = field.replace("pid:", "");


            if !field1.len() == 9 {
                return false;
            } 

        }
    } 


    true
}

fn main() {
    let data = read_to_string("input.txt").unwrap();
    let single_passports: Vec<String> = data.split("\n\n").map(|line| line.to_string()).collect();
    let required: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid: i32 = 0;

    for passport in single_passports {
        if check(passport, &required) {
            valid += 1;
        }
    }

    println!("{valid}");
   
}
