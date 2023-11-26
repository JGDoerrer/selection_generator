CXX=g++
RM=rm -f
CXXFLAGS=-std=c++20 -O3 -march=native
CXXFLAGS_DEBUG=-Wall -Wextra -Wconversion -Wno-unknown-pragmas -Wmaybe-uninitialized -Wshadow -fsanitize=undefined,address -D_GLIBCXX_DEBUG
SRCS=main.cpp
OBJS=$(SRCS:.cpp=.o)
TARGET=output.out

normal: $(OBJS)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(OBJS)
	$(RM) $(OBJS)
	./$(TARGET)

nauty: $(OBJS)
	$(CXX) $(CXXFLAGS) -DUSE_NAUTY -o $(TARGET) $(OBJS) ../nauty2_8_8/nauty.a
	$(RM) $(OBJS)
	./$(TARGET)

debugNormal: $(OBJS)
	$(CXX) $(CXXFLAGS) $(CXXFLAGS_DEBUG) -o $(TARGET) $(OBJS)
	$(RM) $(OBJS)
	./$(TARGET)

debugNauty: $(OBJS)
	$(CXX) $(CXXFLAGS) $(CXXFLAGS_DEBUG) -DUSE_NAUTY -o $(TARGET) $(OBJS) ../nauty2_8_8/nauty.a
	$(RM) $(OBJS)
	./$(TARGET)

clean:
	$(RM) $(OBJS)
	$(RM) $(TARGET)