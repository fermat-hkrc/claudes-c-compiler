static int f1(int *p, int *q) { *p += 271; return *q * 2; }
static int f2(int *p, int *q) { *p += 966; return *q * 2; }
static int f3(int *p, int *q) { *p += 760; return *q * 2; }
static int f4(int *p, int *q) { *p += 839; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 390u; }
    sum += f1(&cells[0], &cells[2]);
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    sum += f2(&cells[4], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 958u; }
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 711u; }
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 687u; }
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 802u; }
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 867u; }
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 440u; }
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += f3(&cells[7], &cells[4]);
    arr[6 % 6] = 562; cells[6] += arr[6 % 6];
    cells[1] = cells[5] + 25;
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 292u; }
    sum += f4(&cells[0], &cells[7]);
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    cells[7] = cells[1] + 977;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }