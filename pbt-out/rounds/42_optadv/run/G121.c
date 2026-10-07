static int f1(int *p, int *q) { *p += 439; return *q * 2; }
static int f2(int *p, int *q) { *p += 76; return *q * 2; }
static int f3(int *p, int *q) { *p += 692; return *q * 2; }
static int f4(int *p, int *q) { *p += 461; return *q * 2; }
static int f5(int *p, int *q) { *p += 319; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    cells[2] = cells[4] + 436;
    cells[4] = cells[1] + 942;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 146u; }
    sum += f1(&cells[6], &cells[4]);
    arr[4 % 6] = 325; cells[3] += arr[4 % 6];
    sum += f2(&cells[0], &cells[5]);
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[5 % 6] = 80; cells[0] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[4] = cells[5] + 628;
    sum += f3(&cells[4], &cells[0]);
    sum += f4(&cells[6], &cells[1]);
    sum += f5(&cells[6], &cells[4]);
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[4] = cells[6] + 269;
    arr[3 % 6] = 993; cells[1] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    arr[6 % 6] = 267; cells[6] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }