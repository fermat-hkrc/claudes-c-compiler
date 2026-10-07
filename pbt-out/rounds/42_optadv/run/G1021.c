static int f1(int *p, int *q) { *p += 616; return *q * 2; }
static int f2(int *p, int *q) { *p += 180; return *q * 2; }
static int f3(int *p, int *q) { *p += 951; return *q * 2; }
static int f4(int *p, int *q) { *p += 17; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 411u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    arr[4 % 6] = 856; cells[5] += arr[4 % 6];
    sum += f1(&cells[0], &cells[7]);
    sum += f2(&cells[2], &cells[4]);
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    arr[2 % 6] = 228; cells[2] += arr[2 % 6];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    arr[5 % 6] = 379; cells[0] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += f3(&cells[2], &cells[5]);
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 568u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 245u; }
    sum += f4(&cells[5], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }