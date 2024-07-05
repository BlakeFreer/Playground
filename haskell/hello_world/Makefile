FLAGS = -Wall

main.exe: main.hs
	ghc $< -o $@ $(FLAGS)

.PHONY: clean
clean:
	rm -f *.o *.exe *.hi
