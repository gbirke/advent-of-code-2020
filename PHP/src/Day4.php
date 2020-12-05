<?php

class Day4 {
	public function parseInput(string $input): array {
		$allowed = [
    "byr" => 1,
    "iyr"=> 1,
    "eyr"=> 1, 
    "hgt"=> 1, 
    "hcl"=> 1, 
    "ecl"=> 1, 
    "pid"=> 1, 
    "cid"=> 1, 
		];
		$result = [];
		$entries = preg_split("/\n\n/", $input);
		foreach( $entries as $entry ) {
			$words = preg_split('/\s/', $entry);
			$document = [];
			foreach( $words as $word ) {
				$parts = explode(":", $word, 2);
				if (empty($allowed[$parts[0]])) continue;
				$document[$parts[0]] =$parts[1];
			}
			$result[]=$document;
		}
		return $result;
	}

	public function countValidDocuments(array $documents): int {
		return count(array_filter( $documents, function($document) {
			switch(count($document)) {
			case 8:
				$valid = true;
				break;
			case 7:
				$valid = empty($document['cid']);
				break;
			default:
				return false;
			}
			foreach( $document as $k => $v ) {
				$check = $this->isValidAttribute($k, $v);
				$valid = $valid & $check;
			}
			return $valid;
		} ));
	}

	private function isValidAttribute(string $key, string $value): bool {
		switch($key) {
		case 'byr':
			$birthYear = intval($value);
			return $birthYear >= 1920 && $birthYear <= 2002;
		case 'iyr':
			$issueYear = intval($value);
			return $issueYear >= 2010 && $issueYear <= 2020;
		case 'eyr':
			$expirationYear = intval($value);
			return $expirationYear >= 2020 && $expirationYear <= 2030;
		case 'hgt':
			if ( !preg_match('/^(\d+)(cm|in)/', $value, $matches)) return false;
			$height = intval($matches[1]);
			switch($matches[2]) {
			case 'cm':				
				return $height >= 150 && $height <= 193; 
			case 'in':				
				return $height >= 59 && $height <= 76;
			}
			return false;
		case 'hcl':
			return preg_match('/#[0-9a-f]{6}/', $value);
		case 'ecl':
			return preg_match('/amb|blu|brn|gry|grn|hzl|oth/', $value);
		case 'pid':
			return preg_match('/^\d{9}$/', $value);
		case 'cid':
			return true;
		}
	}
}
