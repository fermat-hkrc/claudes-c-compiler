_Thread_local int tl_counter = 5;
_Thread_local int tl_uninit;
void *get_addr(void) { return (void *)&tl_counter; }
int main(void) {
  tl_counter += 2; tl_uninit = tl_counter * 3;
  printf("%d %d %d\n", tl_counter, tl_uninit, &tl_counter == get_addr());
  return 0;
}
