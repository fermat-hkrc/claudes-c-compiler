static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    static int f1(int *p, int *q) { *p += 495; return *q * 2; }
    sum += f1(&cells[6], &cells[7]);
    static int f2(int *p, int *q) { *p += 673; return *q * 2; }
    sum += f2(&cells[2], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 784u; }
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 834u; }
    cells[3] = cells[6] + 829;
    cells[5] = cells[7] + 242;
    arr[7 % 6] = 162; cells[2] += arr[7 % 6];
    arr[1 % 6] = 472; cells[3] += arr[1 % 6];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }