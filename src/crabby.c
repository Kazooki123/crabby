// crabby.c
#include "crabby.h"
#include "crabbylib.h"

Lexer* new_lexer(const char* input) {
    Lexer* lexer = (Lexer*)malloc(sizeof(Lexer));
    lexer->input = input;
    lexer->position = 0;
    lexer->read_position = 0;
    lexer->ch = input[0];
    return lexer;
}

static void read_char(Lexer* lexer) {
    if (lexer->read_position >= strlen(lexer->input)) {
        lexer->ch = '\0';
    } else {
        lexer->ch = lexer->input[lexer->read_position];
    }
    lexer->position = lexer->read_position;
    lexer->read_position++;
}

static void skip_whitespace(Lexer* lexer) {
    while (lexer->ch == ' ' || lexer->ch == '\t' || lexer->ch == '\n' || lexer->ch == '\r') {
        read_char(lexer);
    }
}

static char* read_string(Lexer* lexer) {
    int start_position = lexer->position + 1;
    do {
        read_char(lexer);
    } while (lexer->ch != '"' && lexer->ch != '\0');

    int length = lexer->position - start_position;
    char* str = (char*)malloc(length + 1);
    strncpy(str, &lexer->input[start_position], length);
    str[length] = '\0';
    return str;
}

Token* next_token(Lexer* lexer) {
    Token* token = (Token*)malloc(sizeof(Token));

    skip_whitespace(lexer);

    switch (lexer->ch) {
        case '"':
            token->type = TOKEN_STRING;
            token->value = read_string(lexer);
            break;
        case '\0':
            token->type = TOKEN_EOF;
            token->value = strdup("EOF");
            break;
        default:
            if (strncmp(&lexer->input[lexer->position], "print", 5) == 0) {
                token->type = TOKEN_PRINT;
                token->value = strdup("print");
                lexer->position += 4;
                lexer->read_position = lexer->position + 1;
            } else {
                token->type = TOKEN_EOF;
                token->value = strdup("EOF");
            }
    }

    read_char(lexer);
    return token;
}

void print_token(Token* token) {
    printf("Token { type: %d, value: %s }\n", token->type, token->value);
}

void free_token(Token* token) {
    free(token->value);
    free(token);
}

void interpret(const char* input) {
    Lexer* lexer = new_lexer(input);
    Token* token;

    while ((token = next_token(lexer))->type != TOKEN_EOF) {
        if (token->type == TOKEN_PRINT) {
            free_token(token);
            token = next_token(lexer);
            if (token->type == TOKEN_STRING) {
                crabby_print(token->value);
            } else {
                printf("Error: Expected string after 'print'\n");
            }
        } else {
            printf("Error: Unexpected token\n");
        }
        free_token(token);
    }

    free_token(token);
    free(lexer);
}

// New function to read file contents
char* read_file(const char* filename) {
    FILE* file = fopen(filename, "r");
    if (file == NULL) {
        printf("Error: Could not open file %s\n", filename);
        return NULL;
    }

    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    fseek(file, 0, SEEK_SET);

    char* buffer = (char*)malloc(file_size + 1);
    if (buffer == NULL) {
        printf("Error: Memory allocation failed\n");
        fclose(file);
        return NULL;
    }

    size_t read_size = fread(buffer, 1, file_size, file);
    buffer[read_size] = '\0';

    fclose(file);
    return buffer;
}

int main(int argc, char* argv[]) {
    if (argc != 2) {
        printf("Usage: %s <filename.cb>\n", argv[0]);
        return 1;
    }

    const char* filename = argv[1];
    const char* extension = strrchr(filename, '.');
    if (extension == NULL || strcmp(extension, ".cb") != 0) {
        printf("Error: File must have a .cb extension\n");
        return 1;
    }

    char* input = read_file(filename);
    if (input == NULL) {
        return 1;
    }

    interpret(input);

    free(input);
    return 0;
}