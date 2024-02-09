/**
 * @author Blake Freer
 * @date 2024-02-07
 * @brief Solve the interview problem posed by LowLevelLearning in the video
 * https://www.youtube.com/watch?v=V2h_hJ5MSpY
 * The problem is to write a program that determines if the stack grows up or
 * down.
 **/

#include <stdio.h>

typedef unsigned long long ptr_t;

/// @brief Allocate a variable on the stack.
/// @return Address of that variable
ptr_t inner() {
	int temp = 0;
	
	return (ptr_t)&temp;
}

int main(void) {
	int temp = 0;

	// Get the address of the arbitrary variable
	ptr_t x = (ptr_t)&temp;

	// Get the address of a variable from a function pushed onto the stack
	// after main()
	ptr_t y = inner();

	if(y > x) {
		// inner() variable has a greater address
		printf("Stack grows up!\n");
	} else {
		// inner() variable has a lower address
		printf("Stack grows down!\n");
	}

	return 0;
 }