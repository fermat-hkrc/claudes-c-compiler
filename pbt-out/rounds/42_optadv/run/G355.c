static int f1(int *p, int *q) { *p += 299; return *q * 2; }
static int f2(int *p, int *q) { *p += 24; return *q * 2; }
static int f3(int *p, int *q) { *p += 714; return *q * 2; }
static int f4(int *p, int *q) { *p += 640; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[5], &cells[3]);
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += f2(&cells[4], &cells[4]);
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 150u; }
    sum += f3(&cells[1], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[5] = cells[7] + 141;
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 133u; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    cells[1] = cells[0] + 211;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[4] = cells[3] + 819;
    cells[0] = cells[4] + 794;
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    cells[3] = cells[3] + 491;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f4(&cells[4], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }