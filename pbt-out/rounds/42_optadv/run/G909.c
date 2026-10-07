static int f1(int *p, int *q) { *p += 977; return *q * 2; }
static int f2(int *p, int *q) { *p += 895; return *q * 2; }
static int f3(int *p, int *q) { *p += 966; return *q * 2; }
static int f4(int *p, int *q) { *p += 80; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    cells[2] = cells[6] + 940;
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 350u; }
    cells[4] = cells[0] + 533;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f1(&cells[3], &cells[6]);
    arr[7 % 6] = 529; cells[2] += arr[7 % 6];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    cells[3] = cells[5] + 428;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[4 % 6] = 77; cells[3] += arr[4 % 6];
    sum += f2(&cells[7], &cells[3]);
    sum += f3(&cells[6], &cells[5]);
    sum += f4(&cells[7], &cells[7]);
    cells[1] = cells[3] + 116;
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    cells[2] = cells[2] + 956;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }