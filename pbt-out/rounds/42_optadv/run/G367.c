static int f1(int *p, int *q) { *p += 685; return *q * 2; }
static int f2(int *p, int *q) { *p += 706; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[4] = cells[3] + 432;
    cells[0] = cells[6] + 154;
    cells[0] = cells[3] + 688;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    cells[0] = cells[4] + 904;
    sum += f1(&cells[3], &cells[1]);
    sum += f2(&cells[3], &cells[3]);
    arr[1 % 6] = 204; cells[1] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[2] = cells[2] + 218;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 883u; }
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 396u; }
    cells[2] = cells[4] + 128;
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }