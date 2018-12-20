#include <stdio.h>
#include <stdlib.h>

int main ()
{
  unsigned long long r0, r1, r2, r3, r5;

  r0 = 0;
  r5 = 10551329;

  for (r3 = 1; r3 <= r5; r3++)
    if (r5 / r3 * r3 == r5)
        r0 += r3;

  printf("%lld", r0);
  exit(0);
}
