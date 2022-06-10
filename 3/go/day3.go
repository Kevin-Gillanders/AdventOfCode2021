package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sync"
)


var wg sync.WaitGroup



func logError(err error) {
	if(err != nil){
		log.Fatal(err)
	}
}


func binStrToInt(s string) uint64{
	s = reverseString(s)
	exp := 1
	total := 0

	for _, v := range s {
		if int(v) == 49{
			total += exp 
		}
		exp = exp * 2
	}

	return uint64(total)

}


func reverseString(s string)string{


	tmpStringSlice := []rune (s)
	tmpString := []rune{}
	for i := len(s) - 1; i >= 0; i--{
		tmpString = append(tmpString, tmpStringSlice[i]);
	} 

	return string(tmpString)

}

func GetMostCommon(scanner *bufio.Scanner) []int{

	mostCommon := []int{}
	for scanner.Scan(){
		binStr := scanner.Text()
		if len(mostCommon) == 0{
			mostCommon = make([] int, len(binStr))
		}
		// fmt.Println(binStr)

		// binStr = reverseString(binStr)
		// fmt.Println(binStr)

		for i, x := range binStr{
			var val int
			if x == '1'{
				val = 1
			} else {
				val = -1
			}
			mostCommon[i] += val
		}

		// x := binStrToInt(binStr)
		// fmt.Println(x)	

		// fmt.Println("======================")	
	}
	return mostCommon
}

func part1(scanner *bufio.Scanner){
	
	mostCommon := GetMostCommon(scanner)

	fmt.Println("======================")	
	fmt.Println(mostCommon)
	fmt.Println("======================")	

	gammaRateBin :=   make([] rune, len(mostCommon))
	epsilonRateBin := make([] rune, len(mostCommon))
	for i, x := range mostCommon{
		if x > 0{
			gammaRateBin[i]   = '1'
			epsilonRateBin[i] = '0'
		} else{
			gammaRateBin[i]   = '0'
			epsilonRateBin[i] = '1'
		}
	}
	fmt.Println(string(gammaRateBin), string(epsilonRateBin))
	gammaRate := binStrToInt(string(gammaRateBin))
	epsilonRate := binStrToInt(string(epsilonRateBin))
	fmt.Println("======================")	
	fmt.Printf("gamma %v | epsi %v\n", gammaRate, epsilonRate)
	fmt.Println("gamma * epsi : ", gammaRate * epsilonRate)
	fmt.Println("======================")	
}


func part2(data [] string, pos int) int{

	mostCommon := 0

	for _, str := range data{
		

		var val int
		if str[pos] == '1' {
			val = 1
		} else {
			val = -1
		}

		mostCommon += val
		
	}
	// fmt.Println("mostCommon ",  mostCommon)

	return mostCommon

	// mostCandidates := []string{} 
	// lessCandidates := []string{}
	// for i, val := range mostCommon{
	// 	if val > 1 {

	// 	}
	// }
}


func part2Compare(dataToCheck [] string, mostCommon bool, ch chan <- string){
	
	defer close(ch)
	defer wg.Done()

	var runeResult rune
	var runeA rune
	var runeB rune
	
	if mostCommon {

		runeA = '1'
		runeB = '0'

 	} else {

 		runeA = '0'
		runeB = '1'

 	}

	for i := 0; i < len(dataToCheck[0]); i++ {

		occurances := part2(dataToCheck, i)

		if occurances > 0 {
			runeResult  =  runeA
		} else if occurances < 0 {
			runeResult  =  runeB
		} else{
			runeResult  = runeA
		}
		
		dataToCheck = dataContains(dataToCheck, runeResult, i )

		if len(dataToCheck) == 1 {
			// fmt.Println("mostCommon", dataToCheck)
			break
		}

	}
	ch <- dataToCheck[0]
}

func main(){


	f, err := os.Open("input.txt")
	logError(err)
	scanner := bufio.NewScanner(f)

	fmt.Println("===========part 1===========")
	part1(scanner)

	f, err = os.Open("input.txt")
	logError(err)
	scanner = bufio.NewScanner(f)

	fmt.Println("===========part 2===========")
	
	var data []string
	for scanner.Scan(){
		data = append(data, scanner.Text())
	}
	
	wg.Add(2)
	ogrChan := make(chan string)
	csrChan := make(chan string)

	go part2Compare(data, true , ogrChan)
	go part2Compare(data, false, csrChan)
	
	ogrMsg := reverseString(<- ogrChan)
	csrMsg := reverseString(<- csrChan)

	wg.Wait()
	
	lsr := binStrToInt(reverseString(ogrMsg)) * binStrToInt(reverseString(csrMsg))

	fmt.Println("Oxygen generator rating : ", ogrMsg)
	fmt.Println("CO2 scrubber rating  : "   , csrMsg)
	fmt.Println("Oxygen generator rating int : ", binStrToInt(ogrMsg))
	fmt.Println("CO2 scrubber rating  int : "   , binStrToInt(csrMsg))
	fmt.Println("Life support rating  : "   , lsr)
}

func dataContains(data []string, comparison rune, pos int) [] string{
	// fmt.Println("dataContains : ", string(comparison))
	// fmt.Println("dataContains : ", data)
	newData := [] string{}
	for _, x := range data{
		runeSlice := []rune(x)
		if runeSlice[pos] == comparison {
			newData = append(newData, x)
		}
	}
	// fmt.Println("dataContains returned : ", newData)
	// fmt.Println("+++++++++++++++++++++++++++++++++++++++++++")

	return newData
}