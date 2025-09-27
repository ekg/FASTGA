CC = gcc
CFLAGS = -O3 -Wall -Wextra -fPIC -std=c99
LDFLAGS = -shared

TARGET = libfastga.so
STATIC_TARGET = libfastga.a

SOURCES = libfastga.c
OBJECTS = $(SOURCES:.c=.o)

.PHONY: all clean test

all: $(TARGET) $(STATIC_TARGET)

$(TARGET): $(OBJECTS)
	$(CC) $(LDFLAGS) -o $@ $^

$(STATIC_TARGET): $(OBJECTS)
	ar rcs $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

test: $(TARGET)
	$(CC) -o test_c test.c -L. -lfastga -Wl,-rpath,.
	./test_c

clean:
	rm -f $(OBJECTS) $(TARGET) $(STATIC_TARGET) test_c