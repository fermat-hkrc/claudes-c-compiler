static int f1(int *p, int *q) { *p += 499; return *q * 2; }
static int f2(int *p, int *q) { *p += 905; return *q * 2; }
static int f3(int *p, int *q) { *p += 606; return *q * 2; }
static int f4(int *p, int *q) { *p += 205; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[5], &cells[0]);
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 299u; }
    sum += f2(&cells[5], &cells[6]);
    sum += f3(&cells[1], &cells[6]);
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 499u; }
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    arr[0 % 6] = 945; cells[5] += arr[0 % 6];
    arr[1 % 6] = 927; cells[2] += arr[1 % 6];
    sum += f4(&cells[6], &cells[5]);
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 647u; }
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[7 % 6] = 631; cells[4] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 967u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }