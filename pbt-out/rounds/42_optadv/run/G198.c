static int f1(int *p, int *q) { *p += 698; return *q * 2; }
static int f2(int *p, int *q) { *p += 333; return *q * 2; }
static int f3(int *p, int *q) { *p += 791; return *q * 2; }
static int f4(int *p, int *q) { *p += 40; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f1(&cells[1], &cells[1]);
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 166u; }
    arr[7 % 6] = 273; cells[7] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    sum += f2(&cells[4], &cells[0]);
    arr[7 % 6] = 934; cells[3] += arr[7 % 6];
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[5] = cells[4] + 633;
    cells[2] = cells[5] + 606;
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 404u; }
    sum += f3(&cells[5], &cells[5]);
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    cells[4] = cells[4] + 897;
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 487u; }
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 256u; }
    cells[5] = cells[1] + 543;
    arr[7 % 6] = 354; cells[3] += arr[7 % 6];
    sum += f4(&cells[7], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }