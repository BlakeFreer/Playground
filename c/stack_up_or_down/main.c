/**
 * @author Blake Freer
 * @date 2024-02-07
 * @brief Solve the interview problem posed by LowLevelLearning in the video
 * https://www.youtube.com/watch?v=V2h_hJ5MSpY
 * The problem is to write a program that determines if the stack grows up or
 * down,
 **/

#include <stdio.h>

typedef unsigned long long ptr_t;

ptr_t inner() {
	int x = 0;
	
	return (ptr_t)&x;
}

int main(int argc, char** argv) {

	int x = 0;
	int y = inner();

	if(y > x) {
		printf("Stack grows up!\n");
	} else {
		printf("Stack grows down!\n");
	}

	return 0;
 }