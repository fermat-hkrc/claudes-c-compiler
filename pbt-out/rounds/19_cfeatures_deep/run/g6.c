int sum(int n, int a[n]) { int t = 0; for (int i = 0; i < n; i++) t += a[i]; return t; }  // VLA param
int main(void) { int a[4] = { 1, 2, 3, 4 }; printf("%d\n", sum(4, a)); return 0; }
