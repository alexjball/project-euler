CFLAGS=-Wpedantic

build/problem% : problem%.cpp
	g++ $< $(CFLAGS) -o $@

format:
	clang-format --style=file -i *.cpp

clean:
	rm build/*