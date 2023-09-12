CC = clang
CFLAGS = -std=c++20
TARGET = output
SOURCES = Vector.cpp Pixel.cpp Hit.cpp Triangle.cpp TriObject.cpp

all: $(TARGET)

$(TARGET): $(SOURCES)
	$(CC)	$(CFLAGS)	$(SOURCES)	-o	$(TARGET)
clean:
	rm	-f	$(TARGET)
