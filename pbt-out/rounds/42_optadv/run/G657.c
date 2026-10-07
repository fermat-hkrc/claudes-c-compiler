static int f1(int *p, int *q) { *p += 526; return *q * 2; }
static int f2(int *p, int *q) { *p += 461; return *q * 2; }
static int f3(int *p, int *q) { *p += 830; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[0]);
    sum += f2(&cells[4], &cells[5]);
    arr[3 % 6] = 147; cells[4] += arr[3 % 6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += f3(&cells[1], &cells[7]);
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 721u; }
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }