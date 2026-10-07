static int f1(int *p, int *q) { *p += 533; return *q * 2; }
static int f2(int *p, int *q) { *p += 197; return *q * 2; }
static int f3(int *p, int *q) { *p += 368; return *q * 2; }
static int f4(int *p, int *q) { *p += 389; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 3; cells[4] += arr[2 % 6];
    sum += f1(&cells[6], &cells[1]);
    cells[6] = cells[4] + 414;
    sum += f2(&cells[5], &cells[0]);
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 569u; }
    cells[6] = cells[2] + 439;
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    arr[4 % 6] = 283; cells[0] += arr[4 % 6];
    arr[7 % 6] = 135; cells[1] += arr[7 % 6];
    arr[4 % 6] = 681; cells[2] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    cells[4] = cells[6] + 746;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[1] = cells[1] + 797;
    sum += f3(&cells[4], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[6] = cells[7] + 203;
    cells[7] = cells[1] + 427;
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    sum += f4(&cells[3], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }