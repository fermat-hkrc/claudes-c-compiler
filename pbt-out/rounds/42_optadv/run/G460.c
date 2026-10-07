static int f1(int *p, int *q) { *p += 11; return *q * 2; }
static int f2(int *p, int *q) { *p += 627; return *q * 2; }
static int f3(int *p, int *q) { *p += 492; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 74u; }
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    arr[3 % 6] = 959; cells[3] += arr[3 % 6];
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 895u; }
    sum += f1(&cells[1], &cells[1]);
    cells[3] = cells[3] + 382;
    sum += f2(&cells[6], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f3(&cells[5], &cells[3]);
    cells[4] = cells[2] + 534;
    arr[3 % 6] = 780; cells[0] += arr[3 % 6];
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    cells[5] = cells[6] + 841;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }