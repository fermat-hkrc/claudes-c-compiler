static int f1(int *p, int *q) { *p += 86; return *q * 2; }
static int f2(int *p, int *q) { *p += 141; return *q * 2; }
static int f3(int *p, int *q) { *p += 888; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f1(&cells[4], &cells[1]);
    sum += f2(&cells[6], &cells[7]);
    sum += f3(&cells[3], &cells[2]);
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 735u; }
    cells[2] = cells[4] + 987;
    cells[6] = cells[1] + 842;
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    cells[6] = cells[6] + 785;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    arr[5 % 6] = 727; cells[5] += arr[5 % 6];
    cells[3] = cells[3] + 272;
    cells[7] = cells[1] + 957;
    arr[5 % 6] = 91; cells[4] += arr[5 % 6];
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }