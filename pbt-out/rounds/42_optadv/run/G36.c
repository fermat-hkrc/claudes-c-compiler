static int f1(int *p, int *q) { *p += 894; return *q * 2; }
static int f2(int *p, int *q) { *p += 808; return *q * 2; }
static int f3(int *p, int *q) { *p += 402; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    arr[4 % 6] = 239; cells[0] += arr[4 % 6];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[0], &cells[2]);
    arr[3 % 6] = 773; cells[2] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f2(&cells[2], &cells[4]);
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    cells[0] = cells[3] + 98;
    arr[7 % 6] = 58; cells[7] += arr[7 % 6];
    sum += f3(&cells[2], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 982u; }
    arr[5 % 6] = 235; cells[3] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 551u; }
    cells[6] = cells[2] + 205;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }