package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type house struct {
	x     int
	y     int
	santa string
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

func processInstructions(instr []string) [8200]house {
	var houseGrid [8200]house
	santaX := 0
	santaY := 0
	roboX := 0
	roboY := 0
	santaX, santaY = getXYFromNextInstr(instr[0], santaX, santaY)
	roboX, roboY = getXYFromNextInstr(instr[1], roboX, roboY)
	santa := true
	for i := 0; i < len(instr)-1; i++ {
		//fmt.Println(instr[i])
		if santa {
			houseGrid[i] = house{santaX, santaY, "santa"}
			if i+2 < len(instr) {
				santaX, santaY = getXYFromNextInstr(instr[i+2], santaX, santaY)
			}
			santa = false
		} else {
			houseGrid[i] = house{roboX, roboY, "robo"}
			if i+2 < len(instr) {
				roboX, roboY = getXYFromNextInstr(instr[i+2], roboX, roboY)
			}
			santa = true
		}
		fmt.Println(houseGrid[i])
	}
	return houseGrid
}

func previousHouses(houseGrid [8200]house, current int) bool {
	checked := false
	for i := 0; i < current; i++ {
		houseGrid[i].santa = "test"
		houseGrid[current].santa = "test"
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

func drive(filename string) int {
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		return -1
	}

	drunkInstructions := strings.Split(string(data), "")
	houseGrid := processInstructions(drunkInstructions)
	return presentCount(houseGrid, drunkInstructions)
}

func main() {
	let := drive("INPUT")
	fmt.Println(let)
}
