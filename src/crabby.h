// crabby.h
#ifndef CRABBY_H
#define CRABBY_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Token types
typedef enum
{
    TOKEN_PRINT,
    TOKEN_STRING,
    TOKEN_EOF
} TokenType;

// Token structure
typedef struct
{
    TokenType type;
    char *value;
} Token;

// Lexer structure
typedef struct
{
    const char *input;
    int position;
    int read_position;
    char ch;
} Lexer;

// Function declarations
Lexer *new_lexer(const char *input);
Token *next_token(Lexer *lexer);
void print_token(Token *token);
void free_token(Token *token);

// Interpreter function
void interpret(const char *input);

// New function declaration
char *read_file(const char *filename);

#endif // CRABBY_H