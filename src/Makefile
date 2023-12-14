CXX=g++
RM=rm -f
CXXFLAGS=-std=c++20 -O3 -march=native
# CXXFLAGS_DEBUG=-Wall -Wextra -Wconversion -Wno-unknown-pragmas -Wmaybe-uninitialized -Wshadow -fsanitize=undefined,address -D_GLIBCXX_DEBUG
# SRCS=forwardSearch.cpp backwardSearch.cpp
# OBJS=$(SRCS:.cpp=.o)
TARGET=output.out

forwardSearch:
	$(CXX) $(CXXFLAGS) -o $(TARGET) forwardSearch.cpp ../nauty2_8_8/nauty.a
	time ./$(TARGET)

backwardSearch:
	$(CXX) $(CXXFLAGS) -o $(TARGET) backwardSearch.cpp ../nauty2_8_8/nauty.a
	time ./$(TARGET)

clean:
#	$(RM) $(OBJS)
	$(RM) $(TARGET)

# normal: $(OBJS)
# 	$(CXX) $(CXXFLAGS) -o $(TARGET) $(OBJS) ../nauty2_8_8/nauty.a

# debug: $(OBJS)
# 	$(CXX) $(CXXFLAGS) $(CXXFLAGS_DEBUG) -o $(TARGET) $(OBJS) ../nauty2_8_8/nauty.a