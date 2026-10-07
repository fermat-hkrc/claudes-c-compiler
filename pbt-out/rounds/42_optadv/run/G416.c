static int f1(int *p, int *q) { *p += 460; return *q * 2; }
static int f2(int *p, int *q) { *p += 843; return *q * 2; }
static int f3(int *p, int *q) { *p += 652; return *q * 2; }
static int f4(int *p, int *q) { *p += 491; return *q * 2; }
static int f5(int *p, int *q) { *p += 843; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    cells[5] = cells[0] + 338;
    arr[0 % 6] = 192; cells[4] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    cells[6] = cells[0] + 882;
    sum += f1(&cells[0], &cells[3]);
    arr[2 % 6] = 213; cells[7] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f2(&cells[7], &cells[4]);
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += f3(&cells[7], &cells[0]);
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 934u; }
    sum += f4(&cells[3], &cells[0]);
    cells[1] = cells[5] + 855;
    sum += f5(&cells[0], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }