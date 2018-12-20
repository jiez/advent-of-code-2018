#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main()
{
    uint64_t r0, r1, r2, r3, r4, r5;

    r0 = r1 = r2 = r3 = r4 = r5 = 0;
    r4 = 0;

L6:
    r3 = r4 | 65536;
    r4 = 10283511;

L8:
    r1 = r3 & 255;
    r4 += r1;
    r4 &= 16777215;
    r4 *= 65899;
    r4 &= 16777215;

    if (r3 < 256)
        goto L28;

    r3 = r3 / 256;

    goto L8;

L28:
    printf("R4 = %d\n", r4);
    if (r4 == r0)
        exit(0);
    goto L6;
}
