#include <pthread.h>
static long total;
static void *work(void *arg) { for (int i = 0; i < 1000; i++) total += 1; return 0; }
int main(void) {
  pthread_t t1, t2;
  pthread_create(&t1, 0, work, 0); pthread_create(&t2, 0, work, 0);
  pthread_join(t1, 0); pthread_join(t2, 0);
  printf("%ld\n", total);
  return 0;
}
