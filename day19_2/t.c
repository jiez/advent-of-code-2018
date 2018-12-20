#include <stdio.h>
#include <stdlib.h>

int main ()
{
  unsigned long long r0, r1, r2, r3, r5;

  r0 = 0;
  r5 = 10551329;

  r3 = 1;

L2:
  r1 = 1;

L3:
  r2 = r1 * r3;
  if (r2 == r5)
  {
    r2 = 1;
    r0 = r3 + r0;
  }
  else
  {
    r2 = 0;
  }

  r1 = r1 + 1;

  if (r1 > r5)
    r2 = 1;
  else
  {
    r2 = 0;
    goto L3;
  }

  r3 = r3 + 1;
  if (r3 > r5)
  {
    r2 = 1;
    printf("%lld", r0);
    exit(0);
  }
  else
  {
    r2 = 0;
    goto L2;
  }
}
