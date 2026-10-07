static int f1(int *p, int *q) { *p += 340; return *q * 2; }
static int f2(int *p, int *q) { *p += 388; return *q * 2; }
static int f3(int *p, int *q) { *p += 773; return *q * 2; }
static int f4(int *p, int *q) { *p += 771; return *q * 2; }
static int f5(int *p, int *q) { *p += 615; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    cells[6] = cells[3] + 5;
    cells[0] = cells[2] + 609;
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 968u; }
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += f1(&cells[2], &cells[4]);
    sum += f2(&cells[4], &cells[3]);
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 495u; }
    cells[2] = cells[1] + 106;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f3(&cells[3], &cells[2]);
    sum += f4(&cells[7], &cells[7]);
    arr[4 % 6] = 496; cells[4] += arr[4 % 6];
    sum += f5(&cells[3], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }