static int f1(int *p, int *q) { *p += 777; return *q * 2; }
static int f2(int *p, int *q) { *p += 580; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    cells[5] = cells[2] + 250;
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 768u; }
    sum += f1(&cells[3], &cells[1]);
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    sum += f2(&cells[0], &cells[1]);
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    arr[6 % 6] = 893; cells[4] += arr[6 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    arr[2 % 6] = 633; cells[5] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 667u; }
    cells[1] = cells[2] + 888;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }