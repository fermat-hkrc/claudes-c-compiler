static int f1(int *p, int *q) { *p += 889; return *q * 2; }
static int f2(int *p, int *q) { *p += 539; return *q * 2; }
static int f3(int *p, int *q) { *p += 466; return *q * 2; }
static int f4(int *p, int *q) { *p += 515; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[0] + 920;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f1(&cells[6], &cells[1]);
    sum += f2(&cells[5], &cells[5]);
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    sum += f3(&cells[0], &cells[4]);
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    cells[2] = cells[6] + 583;
    arr[1 % 6] = 446; cells[6] += arr[1 % 6];
    cells[1] = cells[1] + 453;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 567u; }
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 441u; }
    arr[1 % 6] = 521; cells[0] += arr[1 % 6];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    arr[3 % 6] = 421; cells[7] += arr[3 % 6];
    arr[7 % 6] = 495; cells[3] += arr[7 % 6];
    sum += f4(&cells[5], &cells[6]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }