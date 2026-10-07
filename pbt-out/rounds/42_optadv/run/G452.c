static int f1(int *p, int *q) { *p += 802; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[4] = cells[3] + 955;
    arr[7 % 6] = 220; cells[7] += arr[7 % 6];
    cells[7] = cells[7] + 3;
    sum += f1(&cells[6], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 947u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 126u; }
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    cells[4] = cells[1] + 490;
    cells[6] = cells[1] + 803;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }