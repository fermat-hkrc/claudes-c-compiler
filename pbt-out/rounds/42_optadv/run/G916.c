static int f1(int *p, int *q) { *p += 45; return *q * 2; }
static int f2(int *p, int *q) { *p += 159; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[1 % 6] = 497; cells[4] += arr[1 % 6];
    cells[3] = cells[1] + 39;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[4 % 6] = 842; cells[0] += arr[4 % 6];
    sum += f1(&cells[4], &cells[0]);
    sum += f2(&cells[7], &cells[2]);
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 48u; }
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    cells[6] = cells[3] + 337;
    cells[7] = cells[0] + 79;
    cells[7] = cells[5] + 68;
    arr[6 % 6] = 625; cells[0] += arr[6 % 6];
    arr[1 % 6] = 163; cells[4] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[2 % 6] = 314; cells[0] += arr[2 % 6];
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }