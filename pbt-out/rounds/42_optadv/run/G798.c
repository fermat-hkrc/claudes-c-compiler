static int f1(int *p, int *q) { *p += 385; return *q * 2; }
static int f2(int *p, int *q) { *p += 9; return *q * 2; }
static int f3(int *p, int *q) { *p += 284; return *q * 2; }
static int f4(int *p, int *q) { *p += 322; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[6]);
    sum += f2(&cells[1], &cells[6]);
    sum += f3(&cells[4], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[1 % 6] = 914; cells[7] += arr[1 % 6];
    cells[7] = cells[3] + 5;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += f4(&cells[2], &cells[3]);
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 500u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }