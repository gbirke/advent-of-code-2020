<?php

class Direction {
	private array $directions = ['E', 'S', 'W', 'N'];
	private int $directionIndex = 0;

	public function getCurrent() {
		return $this->directions[$this->directionIndex];
	}

	public function turnLeft(int $degrees) {
		$moves = $degrees / 90;
		$newIndex = ($this->directionIndex - $moves);
		if ($newIndex < 0) {
			$newIndex = 4 + $newIndex;
		}
		$this->directionIndex = $newIndex;
	}

	public function turnRight(int $degrees) {
		$moves = $degrees / 90;
		$newIndex = ($this->directionIndex + $moves);
		if ($newIndex > 3) {
			$newIndex = $newIndex - 4;
		}
		$this->directionIndex = $newIndex;
	}
}
