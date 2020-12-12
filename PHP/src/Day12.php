<?php

require_once __DIR__ . '/Day12/Ship.php';

class Day12 {
	private array $input;

	public function parseInput(string $input): array {
		$result = [];
		foreach(explode("\n", $input) as $line ) {
			if (!preg_match('/^([NESWFLR])(\d+)/', $line, $matches)) {
				error_log("Line $line did not match!");
				continue;
			}
			$result[] = [$matches[1], intval($matches[2])];
		}
		$this->input = $result;
		return $this->input;
	}

	public function steerShip(array $directions): int {
		$ship = new Ship();
		foreach( $directions as [$dir, $unit] ) {
			$ship->go($dir, $unit);
			var_dump($ship);
		}
		return $ship->getDistance();
	}
}
