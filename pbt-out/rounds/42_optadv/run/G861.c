static int f1(int *p, int *q) { *p += 134; return *q * 2; }
static int f2(int *p, int *q) { *p += 59; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    sum += f1(&cells[7], &cells[3]);
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    cells[4] = cells[1] + 617;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    sum += f2(&cells[2], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[1 % 6] = 65; cells[3] += arr[1 % 6];
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 48u; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    arr[1 % 6] = 510; cells[1] += arr[1 % 6];
    arr[4 % 6] = 630; cells[1] += arr[4 % 6];
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }