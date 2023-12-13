run: build
	./a.out

build: main.cpp
	g++ -std=c++20 $^

test: test.cpp
	g++ -std=c++20 $^ -o test.out
	./test.out

