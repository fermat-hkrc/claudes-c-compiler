static int f1(int *p, int *q) { *p += 376; return *q * 2; }
static int f2(int *p, int *q) { *p += 851; return *q * 2; }
static int f3(int *p, int *q) { *p += 732; return *q * 2; }
static int f4(int *p, int *q) { *p += 132; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 311u; }
    sum += f1(&cells[6], &cells[1]);
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[0], &cells[5]);
    cells[0] = cells[3] + 958;
    arr[2 % 6] = 38; cells[1] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    arr[1 % 6] = 920; cells[2] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    arr[2 % 6] = 291; cells[1] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 0u; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    sum += f3(&cells[1], &cells[2]);
    arr[0 % 6] = 951; cells[6] += arr[0 % 6];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 581u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[7 % 6] = 651; cells[3] += arr[7 % 6];
    sum += f4(&cells[5], &cells[7]);
    arr[5 % 6] = 69; cells[7] += arr[5 % 6];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }