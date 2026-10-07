static int f1(int *p, int *q) { *p += 847; return *q * 2; }
static int f2(int *p, int *q) { *p += 867; return *q * 2; }
static int f3(int *p, int *q) { *p += 133; return *q * 2; }
static int f4(int *p, int *q) { *p += 4; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 330u; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    arr[1 % 6] = 124; cells[7] += arr[1 % 6];
    cells[3] = cells[2] + 247;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 898u; }
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[5], &cells[5]);
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f2(&cells[6], &cells[2]);
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 846u; }
    sum += f3(&cells[0], &cells[1]);
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    arr[3 % 6] = 823; cells[3] += arr[3 % 6];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += f4(&cells[4], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }