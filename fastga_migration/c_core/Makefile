CC = gcc
CFLAGS = -O3 -Wall -Wextra -fPIC -std=c99
LDFLAGS = -shared

# Utility functions library
UTIL_TARGET = libfastga.so
UTIL_STATIC = libfastga.a
UTIL_SOURCES = libfastga.c
UTIL_OBJECTS = $(UTIL_SOURCES:.c=.o)

# Full FastGA library
FULL_TARGET = libfastga_full.so
FULL_STATIC = libfastga_full.a
FULL_SOURCES = fastga_full.c
FULL_OBJECTS = $(FULL_SOURCES:.c=.o)

# All targets
TARGET = $(UTIL_TARGET) $(FULL_TARGET)
STATIC_TARGET = $(UTIL_STATIC) $(FULL_STATIC)
SOURCES = $(UTIL_SOURCES) $(FULL_SOURCES)
OBJECTS = $(UTIL_OBJECTS) $(FULL_OBJECTS)

.PHONY: all clean test

all: $(TARGET) $(STATIC_TARGET)

$(UTIL_TARGET): $(UTIL_OBJECTS)
	$(CC) $(LDFLAGS) -o $@ $^

$(UTIL_STATIC): $(UTIL_OBJECTS)
	ar rcs $@ $^

$(FULL_TARGET): $(FULL_OBJECTS)
	$(CC) $(LDFLAGS) -o $@ $^

$(FULL_STATIC): $(FULL_OBJECTS)
	ar rcs $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

test: $(TARGET)
	$(CC) -o test_c test.c -L. -lfastga -Wl,-rpath,.
	./test_c

clean:
	rm -f $(OBJECTS) $(TARGET) $(STATIC_TARGET) test_c