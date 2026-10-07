static int f1(int *p, int *q) { *p += 798; return *q * 2; }
static int f2(int *p, int *q) { *p += 303; return *q * 2; }
static int f3(int *p, int *q) { *p += 163; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 302u; }
    arr[5 % 6] = 55; cells[2] += arr[5 % 6];
    arr[1 % 6] = 918; cells[1] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 621u; }
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 296u; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 727u; }
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    arr[1 % 6] = 887; cells[7] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[6] = cells[7] + 633;
    cells[3] = cells[7] + 147;
    sum += f1(&cells[6], &cells[0]);
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    arr[2 % 6] = 257; cells[0] += arr[2 % 6];
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 729u; }
    sum += f2(&cells[1], &cells[5]);
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    sum += f3(&cells[4], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }