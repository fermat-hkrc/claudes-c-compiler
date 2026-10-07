static int f1(int *p, int *q) { *p += 39; return *q * 2; }
static int f2(int *p, int *q) { *p += 618; return *q * 2; }
static int f3(int *p, int *q) { *p += 254; return *q * 2; }
static int f4(int *p, int *q) { *p += 203; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[3]);
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    cells[7] = cells[1] + 407;
    cells[5] = cells[2] + 748;
    cells[0] = cells[6] + 68;
    arr[6 % 6] = 845; cells[2] += arr[6 % 6];
    cells[1] = cells[5] + 78;
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 137u; }
    cells[2] = cells[6] + 342;
    sum += f2(&cells[6], &cells[6]);
    cells[1] = cells[1] + 194;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    sum += f3(&cells[7], &cells[2]);
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    sum += f4(&cells[6], &cells[6]);
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    cells[6] = cells[0] + 435;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }