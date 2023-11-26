CXX=g++
RM=rm -f
CXXFLAGS=-std=c++20 -O3
CXXFLAGS_DEBUG=-Wall -Wextra -Wconversion -Wno-unknown-pragmas -Wmaybe-uninitialized -Wshadow -fsanitize=undefined,address -D_GLIBCXX_DEBUG
SRCS=main.cpp
OBJS=$(SRCS:.cpp=.o)
TARGET=output.out

main: $(OBJS)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(OBJS)
	./$(TARGET)

debug: $(OBJS)
	$(CXX) $(CXXFLAGS) $(CXXFLAGS_DEBUG) -o $(TARGET) $(OBJS)
	./$(TARGET)

clean:
	$(RM) $(OBJS)
	$(RM) $(TARGET)