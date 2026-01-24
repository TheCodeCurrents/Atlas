CC		:= gcc
CFLAGS  := -std=c11 -Wall -Wextra -Wpedantic -O2 -Iinclude
LDFLAGS :=

SRC := $(wildcard src/*.c)
OBJ := $(SRC:src/%.c=build/%.o)
BIN := atlas8-sim

all: $(BIN)

$(BIN): $(OBJ)
	$(CC) $(OBJ) -o $@ $(LDFLAGS)

build/%.o: src/%.c
	@mkdir -p build
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf build $(BIN)

.PHONY: all clean