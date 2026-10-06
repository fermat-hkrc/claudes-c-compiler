#include <alloca.h>
int squares(int n) {
  int *v = alloca(n * sizeof(int));
  for (int i = 0; i < n; i++) v[i] = i * i;
  int t = 0; for (int i = 0; i < n; i++) t += v[i];
  return t;
}
int vla_matrix(int r, int c) {
  int m[r][c];
  for (int i = 0; i < r; i++) for (int j = 0; j < c; j++) m[i][j] = i * c + j;
  return m[r-1][c-1];
}
int main(void) { printf("%d %d\n", squares(5), vla_matrix(3, 4)); return 0; }
