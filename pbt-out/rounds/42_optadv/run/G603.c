static int f1(int *p, int *q) { *p += 668; return *q * 2; }
static int f2(int *p, int *q) { *p += 118; return *q * 2; }
static int f3(int *p, int *q) { *p += 921; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 185u; }
    sum += f1(&cells[7], &cells[3]);
    arr[2 % 6] = 452; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f2(&cells[5], &cells[1]);
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    cells[6] = cells[1] + 737;
    sum += f3(&cells[5], &cells[1]);
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    cells[2] = cells[0] + 723;
    arr[0 % 6] = 885; cells[5] += arr[0 % 6];
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 26u; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 153u; }
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 381u; }
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }