static int f1(int *p, int *q) { *p += 553; return *q * 2; }
static int f2(int *p, int *q) { *p += 983; return *q * 2; }
static int f3(int *p, int *q) { *p += 682; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    arr[5 % 6] = 383; cells[6] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    arr[4 % 6] = 921; cells[6] += arr[4 % 6];
    cells[3] = cells[4] + 197;
    cells[2] = cells[7] + 440;
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 832u; }
    sum += f1(&cells[4], &cells[7]);
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += f2(&cells[5], &cells[1]);
    sum += f3(&cells[7], &cells[1]);
    cells[3] = cells[6] + 457;
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }