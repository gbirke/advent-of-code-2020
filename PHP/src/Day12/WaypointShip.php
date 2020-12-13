<?php

class WaypointShip {
	private int $x = 0;
	private int $y = 0;
	private int $waypointX = 10;
	private int $waypointY = 1;

	public function goNorth(int $units) {
		$this->waypointY += $units;
	}

	public function goEast(int $units) {
		$this->waypointX += $units;
	}
	
	public function goSouth(int $units) {
		$this->waypointY -= $units;
	}

	public function goWest(int $units) {
		$this->waypointX -= $units;
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
			$this->x += $this->waypointX * $units;
			$this->y += $this->waypointY * $units;
			break;

		case 'L':
			for ($i = 0; $i < ($units / 90); $i++) {
				$newX = $this->waypointY*-1;
				$newY = $this->waypointX;
				$this->waypointX = $newX;
				$this->waypointY = $newY;
			}
			break;
		case 'R':
			for ($i = 0; $i < ($units / 90); $i++) {
				$newX = $this->waypointY;
				$newY = $this->waypointX*-1;
				$this->waypointX = $newX;
				$this->waypointY = $newY;
			}
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
