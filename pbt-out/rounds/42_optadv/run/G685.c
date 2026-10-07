static int f1(int *p, int *q) { *p += 923; return *q * 2; }
static int f2(int *p, int *q) { *p += 730; return *q * 2; }
static int f3(int *p, int *q) { *p += 469; return *q * 2; }
static int f4(int *p, int *q) { *p += 784; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[5]);
    arr[2 % 6] = 297; cells[3] += arr[2 % 6];
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 748u; }
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[2], &cells[6]);
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 655u; }
    cells[4] = cells[0] + 227;
    sum += f3(&cells[2], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f4(&cells[2], &cells[7]);
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    arr[7 % 6] = 17; cells[3] += arr[7 % 6];
    cells[0] = cells[6] + 791;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 44u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[5] = cells[5] + 139;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }