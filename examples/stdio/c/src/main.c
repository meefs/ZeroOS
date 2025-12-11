#include <stddef.h>
#include <stdio.h>

int main(int argc, char **argv) {
    printf("Hello from C (stdio example)!\n");

    printf("Testing printf: string=%s, int=%d, hex=0x%x, char=%c\n",
           "ZeroOS", 42, 0xDEADBEEF, 'Z');

    // Since we are on bare metal, argc/argv might not be populated correctly by the loader yet.
    printf("argc = %d\n", argc);

    return 0;
}
