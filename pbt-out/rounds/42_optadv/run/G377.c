static int f1(int *p, int *q) { *p += 270; return *q * 2; }
static int f2(int *p, int *q) { *p += 345; return *q * 2; }
static int f3(int *p, int *q) { *p += 55; return *q * 2; }
static int f4(int *p, int *q) { *p += 268; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[3 % 6] = 873; cells[1] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[2] = cells[5] + 850;
    cells[6] = cells[7] + 835;
    sum += f2(&cells[4], &cells[6]);
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 109u; }
    sum += f3(&cells[7], &cells[3]);
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    sum += f4(&cells[1], &cells[5]);
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    arr[2 % 6] = 529; cells[5] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 853u; }
    cells[7] = cells[1] + 962;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }