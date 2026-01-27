CC		:= gcc
CFLAGS  := -std=c11 -Wall -Wextra -Wpedantic -O2 -Iinclude
LDFLAGS := -lrt

SRC := $(wildcard src/*.c)
BUILDDIR := build
OBJDIR := $(BUILDDIR)/obj
OBJ := $(SRC:src/%.c=$(OBJDIR)/%.o)
BIN := $(BUILDDIR)/atlas8-sim
TEST_BIN := $(BUILDDIR)/atlas8-test

# Filter out test.c from main build
MAIN_SRC := $(filter-out src/test.c, $(SRC))
MAIN_OBJ := $(MAIN_SRC:src/%.c=$(OBJDIR)/%.o)

# Test sources (everything except main.c)
TEST_SRC := $(filter-out src/main.c, $(SRC))
TEST_OBJ := $(TEST_SRC:src/%.c=$(OBJDIR)/%.o)

all: $(BIN)

# Main simulator binary
$(BIN): $(MAIN_OBJ)
	@mkdir -p $(BUILDDIR)
	$(CC) $(MAIN_OBJ) -o $@ $(LDFLAGS)

# Test binary
$(TEST_BIN): $(TEST_OBJ)
	@mkdir -p $(BUILDDIR)
	$(CC) $(TEST_OBJ) -o $@ $(LDFLAGS)

$(OBJDIR)/%.o: src/%.c
	@mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) -c $< -o $@

test: $(TEST_BIN)
	./$(TEST_BIN)

clean:
	rm -rf $(BUILDDIR)

.PHONY: all test clean