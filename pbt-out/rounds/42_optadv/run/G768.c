static int f1(int *p, int *q) { *p += 252; return *q * 2; }
static int f2(int *p, int *q) { *p += 403; return *q * 2; }
static int f3(int *p, int *q) { *p += 827; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[3] = cells[3] + 740;
    sum += f1(&cells[1], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[1 % 6] = 702; cells[2] += arr[1 % 6];
    sum += f2(&cells[3], &cells[4]);
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 793u; }
    arr[1 % 6] = 409; cells[1] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 277u; }
    arr[1 % 6] = 419; cells[3] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    cells[4] = cells[7] + 197;
    arr[1 % 6] = 303; cells[7] += arr[1 % 6];
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 815u; }
    sum += f3(&cells[4], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }