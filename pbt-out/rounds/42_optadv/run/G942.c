static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    arr[6 % 6] = 397; cells[5] += arr[6 % 6];
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 472u; }
    arr[7 % 6] = 83; cells[6] += arr[7 % 6];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    static int f1(int *p, int *q) { *p += 303; return *q * 2; }
    sum += f1(&cells[5], &cells[5]);
    arr[2 % 6] = 534; cells[0] += arr[2 % 6];
    arr[4 % 6] = 823; cells[6] += arr[4 % 6];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    arr[5 % 6] = 241; cells[5] += arr[5 % 6];
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    cells[5] = cells[3] + 303;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[1] = cells[5] + 142;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 953u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }