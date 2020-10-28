package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type Box struct {
	ID     string
	Double bool
	Triple bool
}

type LevenshteinResult struct {
	Box1     Box
	Box2     Box
	Distance int
}

func main() {
	var boxes []Box
	data := getData()
	double := 0
	triple := 0
	var LevenshteinResults []LevenshteinResult

	for _, id := range data {
		b := Box{
			ID:     id,
			Double: containsRuneCount(2, id),
			Triple: containsRuneCount(3, id),
		}
		boxes = append(boxes, b)

		if b.Double {
			double++
		}
		if b.Triple {
			triple++
		}
	}

	var chickenDinner LevenshteinResult
	for _, box := range boxes {
		for _, b := range boxes {
			if box.ID != b.ID {
				lev := LevenshteinResult{
					Box1:     box,
					Box2:     b,
					Distance: levenshtein([]rune(box.ID), []rune(b.ID)),
				}
				LevenshteinResults = append(LevenshteinResults, lev)
			}
		}
	}

	for _, l := range LevenshteinResults {
		if l.Distance == 1 {
			fmt.Println("Distance 1 found on:", l.Box1.ID, l.Box2.ID)
			chickenDinner = l
			continue
		}
	}

	fmt.Println(fmt.Sprintf("End checksum is: %v, doubles: %v, triples: %v", double*triple, double, triple))
	fmt.Println(fmt.Sprintf("Correct box IDs: %v, %v", chickenDinner.Box1.ID, chickenDinner.Box2.ID))
	fmt.Println(fmt.Sprintf("Shared characters are: %v", sharedRunes([]rune(chickenDinner.Box1.ID), []rune(chickenDinner.Box2.ID))))

}

func sharedRunes(input1, input2 []rune) string {
	var result []rune
	for i := 0; i < len(input1); i++ {
		if input1[i] == input2[i] {
			result = append(result, input1[i])
		}
	}
	return string(result)
}

func containsRuneCount(count int, input string) bool {
	runes := []rune(input)
	for _, r := range runes {
		c := 0
		for _, i := range runes {
			if i == r {
				c++
			}
		}
		if c == count {
			fmt.Println("Exceeded required count:", count, "found:", c, "Input:", input, "Tested input:", string(r))
			return true
		}
	}
	fmt.Println("Failed test on:", input)
	return false
}

func getData() []string {
	content, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	return strings.Split(string(content), "\n")
}

func levenshtein(input1, input2 []rune) int {
	length1 := len(input1)
	length2 := len(input2)
	column := make([]int, len(input1)+1)

	for y := 1; y <= length1; y++ {
		column[y] = y
	}
	for x := 1; x <= length2; x++ {
		column[0] = x
		lastkey := x - 1
		for y := 1; y <= length1; y++ {
			oldkey := column[y]
			var incr int
			if input1[y-1] != input2[x-1] {
				incr = 1
			}

			column[y] = minimum(column[y]+1, column[y-1]+1, lastkey+incr)
			lastkey = oldkey
		}
	}
	return column[length1]
}

func minimum(a, b, c int) int {
	if a < b {
		if a < c {
			return a
		}
	} else {
		if b < c {
			return b
		}
	}
	return c
}
