<?php

require_once "src/Day12.php";

$input = file_get_contents("input/2020/day12.txt");
$program = new Day12();

$testinput = <<<TEST
F10
N3
F7
R90
F11
TEST;
$directions = $program->parseInput(trim($input));
printf("Distance: %d\n", $program->steerShip($directions));
