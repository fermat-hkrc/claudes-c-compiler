static int f1(int *p, int *q) { *p += 874; return *q * 2; }
static int f2(int *p, int *q) { *p += 409; return *q * 2; }
static int f3(int *p, int *q) { *p += 917; return *q * 2; }
static int f4(int *p, int *q) { *p += 207; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[5]);
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f2(&cells[1], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 569u; }
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    sum += f3(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[5] = cells[3] + 726;
    cells[4] = cells[3] + 288;
    sum += f4(&cells[1], &cells[0]);
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    arr[6 % 6] = 179; cells[3] += arr[6 % 6];
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    cells[5] = cells[4] + 980;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }