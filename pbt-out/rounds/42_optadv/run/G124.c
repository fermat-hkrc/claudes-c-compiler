static int f1(int *p, int *q) { *p += 74; return *q * 2; }
static int f2(int *p, int *q) { *p += 289; return *q * 2; }
static int f3(int *p, int *q) { *p += 54; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    arr[2 % 6] = 15; cells[6] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[1 % 6] = 103; cells[3] += arr[1 % 6];
    cells[6] = cells[7] + 487;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[1], &cells[6]);
    arr[7 % 6] = 180; cells[1] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    sum += f2(&cells[3], &cells[6]);
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    arr[7 % 6] = 616; cells[1] += arr[7 % 6];
    sum += f3(&cells[5], &cells[5]);
    arr[6 % 6] = 321; cells[7] += arr[6 % 6];
    cells[3] = cells[6] + 259;
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    cells[6] = cells[6] + 624;
    arr[2 % 6] = 171; cells[0] += arr[2 % 6];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 400u; }
    arr[5 % 6] = 22; cells[0] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }