static int f1(int *p, int *q) { *p += 924; return *q * 2; }
static int f2(int *p, int *q) { *p += 813; return *q * 2; }
static int f3(int *p, int *q) { *p += 862; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    cells[0] = cells[5] + 954;
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    sum += f1(&cells[3], &cells[3]);
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    sum += f2(&cells[7], &cells[3]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 567u; }
    cells[6] = cells[5] + 947;
    arr[6 % 6] = 200; cells[1] += arr[6 % 6];
    sum += f3(&cells[1], &cells[3]);
    cells[5] = cells[5] + 866;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }