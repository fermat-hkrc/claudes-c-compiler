static int f1(int *p, int *q) { *p += 559; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    arr[2 % 6] = 9; cells[4] += arr[2 % 6];
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 122u; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += f1(&cells[4], &cells[5]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    cells[0] = cells[7] + 915;
    cells[5] = cells[3] + 356;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    cells[1] = cells[0] + 304;
    cells[5] = cells[3] + 413;
    arr[4 % 6] = 937; cells[4] += arr[4 % 6];
    arr[5 % 6] = 339; cells[4] += arr[5 % 6];
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 925u; }
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 683u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }