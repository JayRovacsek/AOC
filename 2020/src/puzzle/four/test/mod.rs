#![allow(unused)]
use super::*;
use crate::input::read_contents;

#[test]
fn test_solve_part_one() {
    assert_eq!("190", solve_part_one(&read_contents("./input/day_four.txt")));
}

#[test]
fn test_solve_part_two() {
    assert_eq!("121", solve_part_two(&read_contents("./input/day_four.txt")));
}

#[test]
fn test_passport_is_valid_part_one() {
    let passport_one = Passport::new(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm",
    );

    let passport_two = Passport::new(
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929",
    );

    let passport_three = Passport::new(
        "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm",
    );

    let passport_four = Passport::new(
        "hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in",
    );

    assert_eq!(true, passport_one.is_valid_part_one());
    assert_eq!(false, passport_two.is_valid_part_one());
    assert_eq!(true, passport_three.is_valid_part_one());
    assert_eq!(false, passport_four.is_valid_part_one());
}

#[test]
fn test_passport_is_valid_part_two() {
    let byr_re = Regex::new(r"^(19[2-9][0-9])|(200[0-2])$").unwrap();
    let iyr_re = Regex::new(r"^(20[1][0-9])|(2020)$").unwrap();
    let eyr_re = Regex::new(r"^(20[2][0-9])|(2030)$").unwrap();
    let hgt_re =
        Regex::new(r"(^1[5-8][\d]cm$)|(^19[0-2]cm$)|(^59in$)|(^6[0-9]in$)|(^7[0-6]in$)").unwrap();
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();

    let byr_valid = "2002";
    let byr_invalid = "2003";
    let hgt_valid = "60in";
    let hgt_valid_two = "190cm";
    let hgt_invalid = "190in";
    let hgt_invalid_two = "190";
    let hcl_valid = "#123abc";
    let hcl_invalid = "#123abz";
    let hcl_invalid_two = "123abc";
    let ecl_valid = "brn";
    let ecl_invalid = "wat";
    let pid_valid = "000000001";
    let pid_invalid = "0123456789";

    assert_eq!(true, byr_re.is_match(byr_valid));
    assert_eq!(false, byr_re.is_match(byr_invalid));
    assert_eq!(true, hgt_re.is_match(hgt_valid));
    assert_eq!(true, hgt_re.is_match(hgt_valid_two));
    assert_eq!(false, hgt_re.is_match(hgt_invalid));
    assert_eq!(false, hgt_re.is_match(hgt_invalid_two));
    assert_eq!(true, hcl_re.is_match(hcl_valid));
    assert_eq!(false, hcl_re.is_match(hcl_invalid));
    assert_eq!(false, hcl_re.is_match(hcl_invalid_two));
    assert_eq!(true, ecl_re.is_match(ecl_valid));
    assert_eq!(false, ecl_re.is_match(ecl_invalid));
    assert_eq!(true, pid_re.is_match(pid_valid));
    assert_eq!(false, pid_re.is_match(pid_invalid));

    let invalid_passport_one = Passport::new(
        "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
    );
    let invalid_passport_two = Passport::new(
        "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946",
    );
    let invalid_passport_three = Passport::new(
        "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
    );
    let invalid_passport_four = Passport::new(
        "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
    );

    assert_eq!(false, invalid_passport_one.is_valid_part_two());
    assert_eq!(false, invalid_passport_two.is_valid_part_two());
    assert_eq!(false, invalid_passport_three.is_valid_part_two());
    assert_eq!(false, invalid_passport_four.is_valid_part_two());

    let valid_passport_one = Passport::new(
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f",
    );
    let valid_passport_two = Passport::new(
        "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
    );
    let valid_passport_three = Passport::new(
        "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022",
    );
    let valid_passport_four =
        Passport::new("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719");

    assert_eq!(true, valid_passport_one.is_valid_part_two());
    assert_eq!(true, valid_passport_two.is_valid_part_two());
    assert_eq!(true, valid_passport_three.is_valid_part_two());
    assert_eq!(true, valid_passport_four.is_valid_part_two());
}
