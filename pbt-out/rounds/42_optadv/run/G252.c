static int f1(int *p, int *q) { *p += 127; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[1]);
    cells[0] = cells[4] + 636;
    cells[5] = cells[4] + 66;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 178u; }
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 665u; }
    arr[7 % 6] = 824; cells[0] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }