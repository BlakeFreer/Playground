FLAGS = -Wall

EXE = main.exe

$(EXE): main.hs
	ghc $< -o $@ $(FLAGS)

.PHONY: clean
clean:
	rm -f *.o *.exe *.hi

.PHONY: test
test: $(EXE)
	./$(EXE)
	./$(EXE) -h
	./$(EXE) 10
	./$(EXE) -1
	