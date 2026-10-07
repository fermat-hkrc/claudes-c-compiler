static int f1(int *p, int *q) { *p += 26; return *q * 2; }
static int f2(int *p, int *q) { *p += 433; return *q * 2; }
static int f3(int *p, int *q) { *p += 174; return *q * 2; }
static int f4(int *p, int *q) { *p += 579; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[1]);
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 274u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 252u; }
    sum += f2(&cells[0], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[0] = cells[0] + 391;
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    cells[7] = cells[3] + 434;
    arr[6 % 6] = 902; cells[2] += arr[6 % 6];
    sum += f3(&cells[3], &cells[0]);
    arr[7 % 6] = 273; cells[0] += arr[7 % 6];
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 657u; }
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 910u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f4(&cells[1], &cells[7]);
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 859u; }
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    cells[0] = cells[7] + 754;
    arr[5 % 6] = 470; cells[4] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }