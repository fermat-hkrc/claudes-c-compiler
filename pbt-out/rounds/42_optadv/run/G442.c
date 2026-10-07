static int f1(int *p, int *q) { *p += 170; return *q * 2; }
static int f2(int *p, int *q) { *p += 399; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    cells[2] = cells[3] + 473;
    sum += f1(&cells[2], &cells[5]);
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 490u; }
    sum += f2(&cells[5], &cells[1]);
    arr[7 % 6] = 431; cells[2] += arr[7 % 6];
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 284u; }
    arr[1 % 6] = 75; cells[1] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    cells[7] = cells[3] + 663;
    arr[5 % 6] = 704; cells[2] += arr[5 % 6];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 272u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[3 % 6] = 825; cells[1] += arr[3 % 6];
    cells[2] = cells[5] + 313;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[7] = cells[3] + 474;
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 143u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }