static int f1(int *p, int *q) { *p += 414; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[2] + 763;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 305u; }
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f1(&cells[2], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 419u; }
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 865u; }
    arr[5 % 6] = 686; cells[6] += arr[5 % 6];
    arr[1 % 6] = 987; cells[4] += arr[1 % 6];
    arr[1 % 6] = 583; cells[4] += arr[1 % 6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 823u; }
    arr[6 % 6] = 86; cells[4] += arr[6 % 6];
    cells[2] = cells[3] + 874;
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }