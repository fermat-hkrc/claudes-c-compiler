static int f1(int *p, int *q) { *p += 321; return *q * 2; }
static int f2(int *p, int *q) { *p += 388; return *q * 2; }
static int f3(int *p, int *q) { *p += 788; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f1(&cells[5], &cells[6]);
    cells[1] = cells[7] + 243;
    sum += f2(&cells[5], &cells[3]);
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 239u; }
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 475u; }
    sum += f3(&cells[7], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }