static int f1(int *p, int *q) { *p += 133; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    cells[5] = cells[5] + 930;
    cells[4] = cells[5] + 601;
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    arr[4 % 6] = 248; cells[3] += arr[4 % 6];
    arr[1 % 6] = 139; cells[6] += arr[1 % 6];
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 839u; }
    sum += f1(&cells[7], &cells[2]);
    cells[2] = cells[4] + 20;
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 858u; }
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    cells[4] = cells[0] + 15;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }