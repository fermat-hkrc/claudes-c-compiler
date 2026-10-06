#include <stdio.h>
#define A 1
#define B 2
#if defined(A) && defined(B)
# if A > B
#  define RESULT "A>B"
# elif A < B
#  define RESULT "A<B"
# else
#  define RESULT "A==B"
# endif
#elif defined(A)
# define RESULT "only-A"
#else
# define RESULT "neither"
#endif
#undef A
#undef B
#define A 99
#if defined(A) && A == 99
# define FINAL "redefined-ok"
#else
# define FINAL "redefined-fail"
#endif
int main(void) {
    printf("%s %s\n", RESULT, FINAL);
    return 0;
}
