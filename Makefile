opt?=release

CXX=g++
RM=rm -f
CXXFLAGS=-std=c++20
SRC_FORWARD=${wildcard src/forwardSearch.cpp}
OBJ_FORWARD=${patsubst %.cpp,build/%.o,${SRC_FORWARD}}
SRC_BACKWARD=${wildcard src/backwardSearch.cpp}
OBJ_BACKWARD=${patsubst %.cpp,build/%.o,${SRC_BACKWARD}}
TARGET=output.out
LDFLAGS=nauty2_8_8/nauty.a

ifeq (${opt},release)
CXXFLAGS+=-O3 -march=native
else
CXXFLAGS+=-Wall -Wextra -Wconversion -Wno-unknown-pragmas -Wmaybe-uninitialized -Wshadow -fsanitize=undefined,address -D_GLIBCXX_DEBUG -g
endif

forwardSearch: ${OBJ_FORWARD}
	$(CXX) $(CXXFLAGS) ${OBJ_FORWARD} -o $(TARGET) ${LDFLAGS}
	./$(TARGET)

backwardSearch: ${OBJ_BACKWARD}
	$(CXX) $(CXXFLAGS) ${OBJ_BACKWARD} -o $(TARGET) ${LDFLAGS}
	./$(TARGET)

build/%.o : %.cpp
	mkdir -p ${dir $@}
	${CXX} -o $@ $< -c ${CXXFLAGS}

clean:
	$(RM) -rf build