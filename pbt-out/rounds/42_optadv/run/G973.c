static int f1(int *p, int *q) { *p += 112; return *q * 2; }
static int f2(int *p, int *q) { *p += 594; return *q * 2; }
static int f3(int *p, int *q) { *p += 610; return *q * 2; }
static int f4(int *p, int *q) { *p += 318; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 18; cells[7] += arr[5 % 6];
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 126u; }
    sum += f1(&cells[6], &cells[4]);
    sum += f2(&cells[6], &cells[3]);
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    sum += f3(&cells[3], &cells[2]);
    sum += f4(&cells[4], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }