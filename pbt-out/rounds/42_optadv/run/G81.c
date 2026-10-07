static int f1(int *p, int *q) { *p += 60; return *q * 2; }
static int f2(int *p, int *q) { *p += 30; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    sum += f2(&cells[6], &cells[3]);
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    cells[7] = cells[7] + 682;
    cells[2] = cells[1] + 476;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 440u; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 119u; }
    arr[0 % 6] = 792; cells[7] += arr[0 % 6];
    arr[6 % 6] = 99; cells[6] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }