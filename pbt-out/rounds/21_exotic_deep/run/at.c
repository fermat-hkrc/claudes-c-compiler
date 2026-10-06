#include <stdatomic.h>
atomic_int ctr = 0;
int main(void) {
  atomic_fetch_add(&ctr, 5);
  atomic_fetch_add(&ctr, 3);
  int old = atomic_exchange(&ctr, 100);
  atomic_compare_exchange_strong(&ctr, &(int){100}, 7);
  printf("%d %d %d %d\n", atomic_load(&ctr), old, atomic_is_lock_free(&ctr), ctr += 0);
  return 0;
}
