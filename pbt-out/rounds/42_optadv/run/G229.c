static int f1(int *p, int *q) { *p += 730; return *q * 2; }
static int f2(int *p, int *q) { *p += 981; return *q * 2; }
static int f3(int *p, int *q) { *p += 211; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 199u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += f1(&cells[2], &cells[1]);
    cells[7] = cells[4] + 534;
    arr[5 % 6] = 562; cells[4] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 761u; }
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    cells[6] = cells[5] + 685;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f2(&cells[3], &cells[5]);
    cells[3] = cells[3] + 617;
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    cells[5] = cells[7] + 889;
    sum += f3(&cells[6], &cells[1]);
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 372u; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    arr[7 % 6] = 210; cells[6] += arr[7 % 6];
    arr[5 % 6] = 554; cells[5] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }