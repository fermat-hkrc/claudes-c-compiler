static int f1(int *p, int *q) { *p += 110; return *q * 2; }
static int f2(int *p, int *q) { *p += 491; return *q * 2; }
static int f3(int *p, int *q) { *p += 691; return *q * 2; }
static int f4(int *p, int *q) { *p += 639; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    cells[3] = cells[1] + 857;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 937u; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 646u; }
    cells[4] = cells[5] + 503;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 912u; }
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 326u; }
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 858u; }
    sum += f1(&cells[6], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    cells[3] = cells[6] + 245;
    cells[7] = cells[7] + 944;
    sum += f2(&cells[4], &cells[7]);
    arr[4 % 6] = 487; cells[7] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 296u; }
    sum += f3(&cells[6], &cells[5]);
    arr[4 % 6] = 409; cells[0] += arr[4 % 6];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 509u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f4(&cells[3], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }