<?php

require_once __DIR__ . '/Direction.php';

class Ship {
	private Direction $direction;
	private int $x = 0;
	private int $y = 0;

	public function __construct()
	{
		$this->direction = new Direction();
	}

	public function goNorth(int $units) {
		$this->y += $units;
	}

	public function goEast(int $units) {
		$this->x += $units;
	}
	
	public function goSouth(int $units) {
		$this->y -= $units;
	}

	public function goWest(int $units) {
		$this->x -= $units;
	}

	public function go($direction, $units) {
		switch($direction) {
		case 'N':
			$this->goNorth($units);
			break;

		case 'E':
			$this->goEast($units);
			break;

		case 'S':
			$this->goSouth($units);
			break;

		case 'W':
			$this->goWest($units);
			break;

		case 'F':
			$this->go($this->direction->getCurrent(), $units);
			break;

		case 'L':
			$this->direction->turnLeft($units);
			break;

		case 'R':
			$this->direction->turnRight($units);
			break;

		default:
			throw new InvalidArgumentException("Unknown direction $direction");
		}
	}

	public function getDistance(): int {
		return abs($this->x) + abs($this->y);
	}

}

?>
