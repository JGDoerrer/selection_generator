opt?=release

CXX=g++
RM=rm -f
CXXFLAGS=-std=c++20
SRC_POSET=${wildcard src/poset.cpp}
OBJ_POSET=${patsubst %.cpp,build/%.o,${SRC_POSET}}
SRC_BACKWARD=${wildcard src/main.cpp}
OBJ_BACKWARD=${patsubst %.cpp,build/%.o,${SRC_BACKWARD}}
TARGET=output.out
LDFLAGS=nauty2_8_8/nauty.a

ifeq (${opt},release)
CXXFLAGS+=-O3 -march=native
else
CXXFLAGS+=-Wall -Wextra -Wconversion -Wno-unknown-pragmas -Wmaybe-uninitialized -Wshadow -fsanitize=undefined,address -D_GLIBCXX_DEBUG -g
endif

backwardSearch: ${OBJ_POSET} ${OBJ_BACKWARD}
	$(CXX) $(CXXFLAGS) ${OBJ_BACKWARD} ${OBJ_POSET} -o build/$(TARGET) ${LDFLAGS}
	./build/$(TARGET)

build/%.o : %.cpp
	mkdir -p ${dir $@}
	${CXX} -o $@ $< -c ${CXXFLAGS}

clean:
	$(RM) -rf build