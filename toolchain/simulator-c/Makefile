CC		:= gcc
CFLAGS  := -std=c11 -Wall -Wextra -Wpedantic -O2 -Iinclude
LDFLAGS :=

SRC := $(wildcard src/*.c)
BUILDDIR := build
OBJDIR := $(BUILDDIR)/obj
OBJ := $(SRC:src/%.c=$(OBJDIR)/%.o)
BIN := $(BUILDDIR)/atlas8-sim

all: $(BIN)

$(BIN): $(OBJ)
	@mkdir -p $(BUILDDIR)
	$(CC) $(OBJ) -o $@ $(LDFLAGS)

$(OBJDIR)/%.o: src/%.c
	@mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(BUILDDIR)

.PHONY: all clean