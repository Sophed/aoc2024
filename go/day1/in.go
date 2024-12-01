package main

import (
	"os"
	"strconv"
	"strings"
)

func read(file string) string {
	data, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	return string(data)
}

func parse(contents string) ([]int, []int) {
	leftList := []int{}
	rightList := []int{}
	for _, line := range strings.Split(contents, "\n") {
		if line == "" {
			continue
		}
		pair := strings.Split(line, "   ")
		left, err := strconv.Atoi(pair[0])
		if err != nil {
			panic(err)
		}
		right, err := strconv.Atoi(pair[1])
		if err != nil {
			panic(err)
		}
		leftList = append(leftList, left)
		rightList = append(rightList, right)
	}
	return leftList, rightList
}
