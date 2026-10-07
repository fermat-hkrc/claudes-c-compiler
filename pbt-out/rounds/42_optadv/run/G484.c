static int f1(int *p, int *q) { *p += 818; return *q * 2; }
static int f2(int *p, int *q) { *p += 473; return *q * 2; }
static int f3(int *p, int *q) { *p += 832; return *q * 2; }
static int f4(int *p, int *q) { *p += 308; return *q * 2; }
static int f5(int *p, int *q) { *p += 684; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[3]);
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 834u; }
    arr[2 % 6] = 75; cells[0] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f2(&cells[3], &cells[5]);
    sum += f3(&cells[2], &cells[1]);
    sum += f4(&cells[1], &cells[0]);
    sum += f5(&cells[5], &cells[1]);
    cells[0] = cells[7] + 681;
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 133u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    arr[3 % 6] = 834; cells[4] += arr[3 % 6];
    arr[6 % 6] = 249; cells[6] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    arr[2 % 6] = 758; cells[6] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }