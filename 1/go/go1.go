package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
)

func Section1(scanner bufio.Scanner){

	if !scanner.Scan(){
		return
	}
	previousInput, err := strconv.Atoi(scanner.Text())

	if err != nil{
		log.Fatal(err)
	}

	fmt.Printf("%d (N/A - no previous measurement\n", previousInput)	
	
	var message string
	lCounter := 0

	for scanner.Scan() {
		currentInput, err := strconv.Atoi(scanner.Text())
		if err != nil{
			log.Fatal(err)
		}

		if currentInput > previousInput{
			message = "(increased)"
			lCounter++
		} else {
			message = "(decreased)"
		}

		fmt.Printf("%d %s\n", currentInput, message)	
		previousInput = currentInput

	}


	fmt.Printf("%d Bigger\n", lCounter)	
}


func ReadAnInt(scanner *bufio.Scanner) int{
	intRead, err := strconv.Atoi(scanner.Text())
	if err != nil{
		log.Fatal(err)
	}
	// fmt.Println("read out ", intRead)
	return intRead
}


func Section2(scanner bufio.Scanner){
	if !scanner.Scan(){
		return
	}
	previousInput1 := ReadAnInt(&scanner)
	scanner.Scan()
	previousInput2 := ReadAnInt(&scanner)
	scanner.Scan()
	previousInput3 := ReadAnInt(&scanner)


	fmt.Printf("%d (N/A - no previous measurement\n", sumNums(previousInput1, previousInput2, previousInput3))
	
	var message string
	lCounter := 0

	for scanner.Scan() {
		currentInput1 := previousInput2
		currentInput2 := previousInput3
		currentInput3 := ReadAnInt(&scanner)

		currentSum := sumNums(currentInput1, currentInput2, currentInput3)

		if currentSum > sumNums(previousInput1, previousInput2, previousInput3){
			message = "(increased)"
			lCounter++
		} else {
			message = "(decreased)"
		}

		// fmt.Printf("%d %d %d\n", currentInput1, currentInput2, currentInput3)	
		fmt.Printf("%d %s\n", currentSum, message)	
		previousInput1 = currentInput1
		previousInput2 = currentInput2
		previousInput3 = currentInput3

	}


	fmt.Printf("%d Bigger\n", lCounter)	
}

func sumNums(numbers ... int) int{
	total := 0
	for _, number := range numbers{
		total += number
	}
	return total
}

func main(){

	file, err := os.Open("../input.txt")

	if err != nil{
		log.Fatal(err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	

	// Section1(*scanner)
	file.Seek(0, io.SeekStart)
	Section2(*scanner)

}