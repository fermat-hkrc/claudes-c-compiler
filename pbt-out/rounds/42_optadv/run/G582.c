static int f1(int *p, int *q) { *p += 752; return *q * 2; }
static int f2(int *p, int *q) { *p += 346; return *q * 2; }
static int f3(int *p, int *q) { *p += 846; return *q * 2; }
static int f4(int *p, int *q) { *p += 289; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[0]);
    arr[5 % 6] = 640; cells[1] += arr[5 % 6];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 888u; }
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 795u; }
    sum += f2(&cells[4], &cells[2]);
    arr[7 % 6] = 157; cells[7] += arr[7 % 6];
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 632u; }
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    arr[0 % 6] = 640; cells[0] += arr[0 % 6];
    sum += f3(&cells[3], &cells[4]);
    cells[2] = cells[2] + 319;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f4(&cells[3], &cells[5]);
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }