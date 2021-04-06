package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type house struct {
	x int
	y int
}

func getXYFromNextInstr(instr string, currX int, currY int) (int, int) {
	x := currX
	y := currY
	if instr == ">" {
		x++
	} else if instr == "<" {
		x--
	} else if instr == "^" {
		y++
	} else {
		y--
	}
	return x, y
}

func dHouseFunc() house {
	return house{1, 0}
}

func processInstructions(instr []string) [8200]house {
	var houseGrid [8200]house
	currX := 0
	currY := 0
	for i := 0; i < len(instr); i++ {
		houseGrid[i] = house{currX, currY}
		currX, currY = getXYFromNextInstr(instr[i], currX, currY)
		fmt.Println(houseGrid[i])
	}
	return houseGrid
}

func previousHouses(houseGrid [8200]house, current int) bool {
	checked := false
	for i := 0; i < current; i++ {
		if houseGrid[i] == houseGrid[current] {
			checked = true
			break
		}
	}
	return checked
}

func presentCount(houseGrid [8200]house, instr []string) int {
	count := 0
	for i := 0; i < len(instr); i++ {
		if previousHouses(houseGrid, i) != true {
			count++
		}
	}
	return count
}

func drive() int {
	data, err := ioutil.ReadFile("INPUT")
	if err != nil {
		return -1
	}

	drunkInstructions := strings.Split(string(data), "")
	houseGrid := processInstructions(drunkInstructions)
	return presentCount(houseGrid, drunkInstructions)
}

func main() {
	let := drive()
	fmt.Println(let)
}
