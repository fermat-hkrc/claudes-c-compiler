static int f1(int *p, int *q) { *p += 644; return *q * 2; }
static int f2(int *p, int *q) { *p += 531; return *q * 2; }
static int f3(int *p, int *q) { *p += 60; return *q * 2; }
static int f4(int *p, int *q) { *p += 135; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[1]);
    arr[1 % 6] = 371; cells[2] += arr[1 % 6];
    arr[0 % 6] = 170; cells[2] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    arr[7 % 6] = 994; cells[7] += arr[7 % 6];
    cells[4] = cells[7] + 255;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 448u; }
    arr[7 % 6] = 501; cells[2] += arr[7 % 6];
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 576u; }
    sum += f2(&cells[6], &cells[0]);
    arr[3 % 6] = 212; cells[3] += arr[3 % 6];
    arr[3 % 6] = 760; cells[2] += arr[3 % 6];
    sum += f3(&cells[6], &cells[2]);
    cells[2] = cells[0] + 377;
    sum += f4(&cells[5], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }