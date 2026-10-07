static int f1(int *p, int *q) { *p += 305; return *q * 2; }
static int f2(int *p, int *q) { *p += 5; return *q * 2; }
static int f3(int *p, int *q) { *p += 432; return *q * 2; }
static int f4(int *p, int *q) { *p += 13; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    sum += f1(&cells[7], &cells[6]);
    sum += f2(&cells[4], &cells[1]);
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 198u; }
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 860u; }
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    sum += f3(&cells[6], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    arr[2 % 6] = 387; cells[3] += arr[2 % 6];
    arr[2 % 6] = 600; cells[2] += arr[2 % 6];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 38u; }
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    cells[7] = cells[2] + 918;
    sum += f4(&cells[5], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }