static int f1(int *p, int *q) { *p += 919; return *q * 2; }
static int f2(int *p, int *q) { *p += 701; return *q * 2; }
static int f3(int *p, int *q) { *p += 403; return *q * 2; }
static int f4(int *p, int *q) { *p += 980; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    cells[4] = cells[2] + 388;
    arr[6 % 6] = 289; cells[3] += arr[6 % 6];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 894u; }
    cells[2] = cells[7] + 830;
    cells[0] = cells[5] + 300;
    arr[1 % 6] = 114; cells[0] += arr[1 % 6];
    sum += f1(&cells[1], &cells[7]);
    sum += f2(&cells[5], &cells[6]);
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    arr[7 % 6] = 938; cells[7] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    cells[4] = cells[3] + 321;
    sum += f3(&cells[0], &cells[3]);
    sum += f4(&cells[7], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 754u; }
    cells[4] = cells[5] + 17;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }