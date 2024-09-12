#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
    INTEGER,
    STRING,
    LIST,
    DICTIONARY
} Type;

void encode_integer(long value, char *buffer) {
    sprintf(buffer, "i%lde", value);
}

void encode_string(const char *s, char *buffer) {
    sprintf(buffer, "%zu:%s", strlen(s), s);
}

int main(void) {
    return 0;
}
