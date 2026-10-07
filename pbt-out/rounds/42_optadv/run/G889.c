static int f1(int *p, int *q) { *p += 387; return *q * 2; }
static int f2(int *p, int *q) { *p += 34; return *q * 2; }
static int f3(int *p, int *q) { *p += 929; return *q * 2; }
static int f4(int *p, int *q) { *p += 555; return *q * 2; }
static int f5(int *p, int *q) { *p += 665; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 215u; }
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 12u; }
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 590u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f1(&cells[0], &cells[0]);
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    arr[0 % 6] = 910; cells[5] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 376u; }
    sum += f2(&cells[0], &cells[3]);
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 496u; }
    cells[5] = cells[4] + 731;
    sum += f3(&cells[7], &cells[7]);
    cells[1] = cells[5] + 569;
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 246u; }
    sum += f4(&cells[6], &cells[7]);
    cells[2] = cells[4] + 37;
    sum += f5(&cells[1], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }