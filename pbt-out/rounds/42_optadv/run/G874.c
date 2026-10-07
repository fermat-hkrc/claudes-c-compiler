static int f1(int *p, int *q) { *p += 397; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 393; cells[7] += arr[4 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    cells[6] = cells[3] + 603;
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    arr[3 % 6] = 308; cells[4] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += f1(&cells[5], &cells[2]);
    arr[3 % 6] = 544; cells[1] += arr[3 % 6];
    arr[3 % 6] = 83; cells[2] += arr[3 % 6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 283u; }
    arr[0 % 6] = 695; cells[7] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    arr[4 % 6] = 629; cells[5] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }