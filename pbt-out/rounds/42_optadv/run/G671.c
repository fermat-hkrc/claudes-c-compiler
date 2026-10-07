static int f1(int *p, int *q) { *p += 463; return *q * 2; }
static int f2(int *p, int *q) { *p += 224; return *q * 2; }
static int f3(int *p, int *q) { *p += 441; return *q * 2; }
static int f4(int *p, int *q) { *p += 576; return *q * 2; }
static int f5(int *p, int *q) { *p += 766; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[1]);
    cells[3] = cells[4] + 490;
    arr[1 % 6] = 115; cells[3] += arr[1 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    cells[5] = cells[2] + 481;
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += f2(&cells[0], &cells[2]);
    cells[5] = cells[2] + 908;
    cells[2] = cells[6] + 976;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 402u; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += f3(&cells[1], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f4(&cells[6], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    sum += f5(&cells[7], &cells[3]);
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 757u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }