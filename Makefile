
CC=gcc

SRC_DIR=./src
OBJ_DIR=./obj
BIN_DIR=./bin

CFILES=$(wildcard $(SRC_DIR)/*.c)
HFILES=$(wildcard $(SRC_DIR)/*.h)

FLAGS=-Wall

UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Windows_NT)
    EXE_EXT=.exe
else
    EXE_EXT=
endif

all: $(BIN_DIR)/crabby$(EXE_EXT)

$(BIN_DIR)/crabby$(EXE_EXT): $(CFILES)
	$(CC) -o $@ $^

clean: 
	rm $(OBJ_DIR)/*.o