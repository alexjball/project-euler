CFLAGS=-Wpedantic

run-% : problem%
	./build/problem$*

problem% : problem%.o
	g++ build/$< -o build/$@

%.o : %.cpp
	g++ -c $(CFLAGS) $< -o build/$@

clean:
	rm build/*