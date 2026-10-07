static int f1(int *p, int *q) { *p += 829; return *q * 2; }
static int f2(int *p, int *q) { *p += 958; return *q * 2; }
static int f3(int *p, int *q) { *p += 460; return *q * 2; }
static int f4(int *p, int *q) { *p += 913; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[5]);
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    cells[2] = cells[0] + 406;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 773u; }
    arr[5 % 6] = 566; cells[1] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[2 % 6] = 716; cells[1] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    cells[6] = cells[3] + 381;
    sum += f2(&cells[4], &cells[2]);
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 481u; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 835u; }
    arr[7 % 6] = 70; cells[0] += arr[7 % 6];
    sum += f3(&cells[3], &cells[2]);
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    sum += f4(&cells[5], &cells[7]);
    cells[4] = cells[1] + 906;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }