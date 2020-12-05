<?php

require_once "src/Day4.php";

$input = file_get_contents("input/2020/day4.txt");
$program = new Day4();

$testinput = <<<TEST
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
TEST;
$documents = $program->parseInput(trim($input));
printf("Valid documents: %d\n", $program->countValidDocuments($documents));
