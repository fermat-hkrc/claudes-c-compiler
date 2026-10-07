static int f1(int *p, int *q) { *p += 697; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    arr[1 % 6] = 783; cells[2] += arr[1 % 6];
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    arr[7 % 6] = 98; cells[1] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    cells[2] = cells[4] + 915;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 802u; }
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    sum += f1(&cells[4], &cells[6]);
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 749u; }
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }