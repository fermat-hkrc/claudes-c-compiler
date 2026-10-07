static int f1(int *p, int *q) { *p += 106; return *q * 2; }
static int f2(int *p, int *q) { *p += 420; return *q * 2; }
static int f3(int *p, int *q) { *p += 142; return *q * 2; }
static int f4(int *p, int *q) { *p += 309; return *q * 2; }
static int f5(int *p, int *q) { *p += 236; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[5] + 267;
    sum += f1(&cells[6], &cells[4]);
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 30u; }
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f2(&cells[5], &cells[6]);
    sum += f3(&cells[0], &cells[6]);
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    arr[4 % 6] = 895; cells[2] += arr[4 % 6];
    arr[3 % 6] = 683; cells[6] += arr[3 % 6];
    sum += f4(&cells[0], &cells[6]);
    arr[1 % 6] = 774; cells[2] += arr[1 % 6];
    sum += f5(&cells[6], &cells[3]);
    cells[6] = cells[0] + 646;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 145u; }
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    cells[1] = cells[4] + 577;
    cells[3] = cells[1] + 504;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }