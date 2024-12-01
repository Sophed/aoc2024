package main

import (
	"math"
	"slices"
)

func main() {
	contents := read("input.txt")
	part1(contents)
	part2(contents)
}

func part1(contents string) {
	leftList, rightList := parse(contents)
	total := 0
	for range leftList {
		ml := minList(leftList)
		mr := minList(rightList)
		leftList = remove(leftList, ml)
		rightList = remove(rightList, mr)
		distance := max(ml, mr) - min(ml, mr)
		total += distance
	}
	println("part 1:", total)
}

func part2(contents string) {
	leftList, rightList := parse(contents)
	total := 0
	occurences := make(map[int]int)
	for _, el := range leftList {
		occurences[el]++
	}
	for _, el := range rightList {
		total += el * occurences[el]
	}
	println("part 2:", total)
}

func remove(list []int, target int) []int {
	for i, el := range list {
		if el == target {
			return append(list[:i], list[i+1:]...)
		}
	}
	return nil
}

func minList(list []int) int {
	slices.Sort(list)
	return list[0]
}

func max(x, y int) int {
	return int(math.Max(float64(x), float64(y)))
}

func min(x, y int) int {
	return int(math.Min(float64(x), float64(y)))
}
