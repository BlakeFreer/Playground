FLAGS = -I include
COMPILER = g++

hello.exe: greet.o main.o
	$(COMPILER) $^ -o $@

main.o: main.cc
	$(COMPILER) $^ -c $(FLAGS)

greet.o: include/greet.cpp
	$(COMPILER) $^ -c $(FLAGS)

clean:
	rm -rf *.o *.exe