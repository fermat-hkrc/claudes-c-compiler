static int f1(int *p, int *q) { *p += 523; return *q * 2; }
static int f2(int *p, int *q) { *p += 335; return *q * 2; }
static int f3(int *p, int *q) { *p += 860; return *q * 2; }
static int f4(int *p, int *q) { *p += 56; return *q * 2; }
static int f5(int *p, int *q) { *p += 221; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    cells[0] = cells[7] + 717;
    sum += f1(&cells[0], &cells[7]);
    sum += f2(&cells[3], &cells[3]);
    cells[5] = cells[3] + 95;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 499u; }
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    sum += f3(&cells[1], &cells[7]);
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    sum += f4(&cells[4], &cells[6]);
    sum += f5(&cells[4], &cells[5]);
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 627u; }
    arr[2 % 6] = 135; cells[5] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 830u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }