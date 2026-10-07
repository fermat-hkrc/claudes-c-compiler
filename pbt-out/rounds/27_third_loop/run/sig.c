#include <stdio.h>
#include <signal.h>
#include <setjmp.h>
#include <unistd.h>
static jmp_buf alarm_env;
static void handler(int sig) { longjmp(alarm_env, 1); }
int main(void) {
    signal(SIGALRM, handler);
    int v = setjmp(alarm_env);
    if (v == 0) {
        alarm(1);
        pause();  // Will be interrupted by SIGALRM
        printf("not reached\n");
        return 1;
    }
    printf("caught alarm\n");
    return 0;
}
