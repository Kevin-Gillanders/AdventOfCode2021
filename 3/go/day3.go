package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func logError(err error) {
	if(err != nil){
		log.Fatal(err)
	}
}


func binStrToInt(s string) uint64{
	exp := 1
	total := 0

	for _, v := range s{
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


func main(){


	f, err := os.Open("input.txt")

	logError(err)

	scanner := bufio.NewScanner(f)

	for scanner.Scan(){
		binStr := scanner.Text()
		fmt.Println(binStr)

		binStr = reverseString(binStr)
		fmt.Println(binStr)

		x := binStrToInt(binStr)
		fmt.Println(x)	

		fmt.Println("======================")	
	}

	
}