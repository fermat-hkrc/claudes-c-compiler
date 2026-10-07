static int f1(int *p, int *q) { *p += 631; return *q * 2; }
static int f2(int *p, int *q) { *p += 697; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    arr[1 % 6] = 656; cells[7] += arr[1 % 6];
    arr[0 % 6] = 930; cells[7] += arr[0 % 6];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 503u; }
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 919u; }
    sum += f1(&cells[7], &cells[7]);
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[2] = cells[1] + 543;
    cells[0] = cells[2] + 754;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 148u; }
    arr[2 % 6] = 954; cells[4] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 147u; }
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    arr[5 % 6] = 584; cells[6] += arr[5 % 6];
    sum += f2(&cells[3], &cells[3]);
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }