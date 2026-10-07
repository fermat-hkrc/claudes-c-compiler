static int f1(int *p, int *q) { *p += 105; return *q * 2; }
static int f2(int *p, int *q) { *p += 647; return *q * 2; }
static int f3(int *p, int *q) { *p += 264; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 731; cells[5] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[5], &cells[1]);
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 499u; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    cells[2] = cells[0] + 31;
    sum += f2(&cells[6], &cells[3]);
    sum += f3(&cells[7], &cells[5]);
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    arr[6 % 6] = 577; cells[7] += arr[6 % 6];
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 976u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    arr[2 % 6] = 194; cells[1] += arr[2 % 6];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }