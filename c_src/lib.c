#include <stdio.h>

void c_hello(void) {
    printf("Hello from C!\n");
    printf("Number test: %f\n", 3.141592);
}

void c_error(void) {
    fprintf(stderr, "stderr from C!\n");
    fprintf(stdout, "stdout from C!\n");
}
