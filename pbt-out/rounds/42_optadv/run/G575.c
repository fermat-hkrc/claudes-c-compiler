static int f1(int *p, int *q) { *p += 222; return *q * 2; }
static int f2(int *p, int *q) { *p += 862; return *q * 2; }
static int f3(int *p, int *q) { *p += 749; return *q * 2; }
static int f4(int *p, int *q) { *p += 668; return *q * 2; }
static int f5(int *p, int *q) { *p += 280; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[3] + 756;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 216u; }
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 806u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 770u; }
    sum += f1(&cells[5], &cells[0]);
    sum += f2(&cells[4], &cells[2]);
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += f3(&cells[5], &cells[4]);
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    arr[0 % 6] = 354; cells[4] += arr[0 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 887u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f4(&cells[0], &cells[5]);
    sum += f5(&cells[6], &cells[6]);
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }