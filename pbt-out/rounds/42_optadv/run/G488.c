static int f1(int *p, int *q) { *p += 609; return *q * 2; }
static int f2(int *p, int *q) { *p += 840; return *q * 2; }
static int f3(int *p, int *q) { *p += 757; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 904u; }
    cells[1] = cells[4] + 6;
    cells[6] = cells[5] + 976;
    cells[2] = cells[2] + 70;
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f1(&cells[0], &cells[3]);
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    arr[3 % 6] = 930; cells[1] += arr[3 % 6];
    sum += f2(&cells[0], &cells[4]);
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f3(&cells[5], &cells[1]);
    arr[2 % 6] = 264; cells[3] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }