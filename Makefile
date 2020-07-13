CFLAGS=-Wpedantic

build/problem% : problem%.cpp
	g++ $< $(CFLAGS) -o $@

clean:
	rm build/*