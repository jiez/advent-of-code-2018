#include <stdio.h>
#include <stdlib.h>

int main ()
{
  unsigned long long r0, r1, r2, r3, r5;

  r0 = 0;
  r5 = 10551329;

  r3 = 1;

  do {
    r1 = 1;
    do {
      r2 = r1 * r3;
      if (r2 == r5)
        r0 += r3;

      r1 += 1;
    } while (r1 <= r5);

    r3 += 1;
  } while (r3 <= r5);

  printf("%lld", r0);
  exit(0);
}
