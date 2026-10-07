static int f1(int *p, int *q) { *p += 741; return *q * 2; }
static int f2(int *p, int *q) { *p += 487; return *q * 2; }
static int f3(int *p, int *q) { *p += 166; return *q * 2; }
static int f4(int *p, int *q) { *p += 772; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    sum += f1(&cells[1], &cells[5]);
    sum += f2(&cells[3], &cells[0]);
    sum += f3(&cells[5], &cells[0]);
    cells[2] = cells[2] + 15;
    cells[7] = cells[5] + 246;
    sum += f4(&cells[2], &cells[5]);
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 815u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }