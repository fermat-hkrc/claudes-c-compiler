int main(void) {
  static void *labels[] = { &&L0, &&L1, &&L2 };
  int acc = 0, i = 0;
  goto *labels[0];
L0: acc += 1; i++; goto *labels[i];
L1: acc += 10; i++; goto *labels[i];
L2: printf("%d\n", acc + i); return 0;
}
